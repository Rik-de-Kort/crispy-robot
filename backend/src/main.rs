// Code based on https://github.com/snapview/tokio-tungstenite/blob/master/examples/echo-server.rs
use derive_more::{Add, Sub, AddAssign, SubAssign};
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq, Add, AddAssign, Sub, SubAssign)]
struct Millimeters(u64); // Max is 1.95 light years, should fit
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Degrees(u16); // Arithmetic modulo 360. Invariant: internal u16 is always <360
impl AddAssign for Degrees {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % 360;
    }
}

impl SubAssign for Degrees {
    fn sub_assign(&mut self, rhs: Self) {
        if self.0 < rhs.0 {
            self.0 = (360 + self.0 - rhs.0) % 360;
        } else {
            self.0 = (self.0 - rhs.0) % 360;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Just verifying degree arithmetic works alright
    #[test]
    fn test_add_degrees() {
        let mut x = Degrees(200);
        x += Degrees(90);
        assert_eq!(x, Degrees(290));
        x += Degrees(300);
        assert_eq!(x, Degrees(30));
    }

    #[test]
    fn test_sub_degrees() {
        let mut x = Degrees(100);
        x -= Degrees(90);
        assert_eq!(x, Degrees(10));
        x -= Degrees(200);
        assert_eq!(x, Degrees(170));
    }
}

// CraneState fits in 2*64 + 3*16 = 176 bits
#[derive(Serialize, Deserialize, Debug)]
struct CraneState {
    swing_rotation: Degrees,
    lift_elevation: Millimeters,
    elbow_rotation: Degrees,
    wrist_rotation: Degrees,
    gripper_open: Millimeters,
}


impl CraneState {
    pub fn execute(&mut self, cmd: CraneCommand) {
        match cmd {
            CraneCommand::RotateSwing { direction, degrees} => {
                match direction {
                    Direction::Positive => self.swing_rotation += degrees,
                    Direction::Negative => self.swing_rotation -= degrees,
                }
            },
            CraneCommand::MoveLift { direction, millimeters} => {
                match direction {
                    Direction::Positive => self.lift_elevation +=  millimeters,
                    Direction::Negative => self.lift_elevation -= millimeters,
                }
            },
            CraneCommand::RotateElbow { direction, degrees} => {
                match direction {
                    Direction::Positive => self.elbow_rotation += degrees,
                    Direction::Negative => self.elbow_rotation -= degrees,
                }
            },
            CraneCommand::RotateWrist { direction, degrees} => {
                match direction {
                    Direction::Positive => self.wrist_rotation += degrees,
                    Direction::Negative => self.wrist_rotation -= degrees,
                }
            },
            CraneCommand::MoveGripper { direction, millimeters} => {
                match direction {
                    Direction::Positive => self.lift_elevation += millimeters,
                    Direction::Negative => self.lift_elevation -= millimeters,
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    Positive,
    Negative,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum CraneCommand {
    RotateSwing{direction: Direction, degrees: Degrees},
    MoveLift{direction: Direction, millimeters: Millimeters},
    RotateElbow{direction: Direction, degrees: Degrees},
    MoveGripper{direction: Direction, millimeters: Millimeters},
    RotateWrist{direction: Direction, degrees: Degrees},
}

// {"type": "RotateSwing", "direction": "Positive", "degrees": 45}

type Crane = Arc<Mutex<CraneState>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let crane = Arc::new(Mutex::new(CraneState {
        swing_rotation: Degrees(45),
        lift_elevation: Millimeters(1000),
        elbow_rotation: Degrees(0),
        wrist_rotation: Degrees(0),
        gripper_open: Millimeters(0),
    }));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, crane.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, crane: Crane) -> Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() {
                            match serde_json::from_str::<CraneCommand>(&msg.into_text()?) {
                                Ok(cmd) => {
                                    info!("{:?}", cmd);
                                    {
                                        let mut crane_state = crane.lock().unwrap();
                                        crane_state.execute(cmd);
                                    }
                                    write.send("Got your command!".into()).await?;
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                    write.send("Error reading command".into()).await?;
                                },
                            }
                        } else if msg.is_close() {
                            break;
                        }
                    },
                    None => break,
                }
            }
            _ = interval.tick() => {
                let msg = {
                    // Todo: error handling here
                    let inner_state = &*crane.lock().unwrap();
                    serde_json::to_string(inner_state).unwrap()
                };
                write.send(msg.into()).await?;
            }
        }
    }
    Ok(())
}
