// Code based on https://github.com/snapview/tokio-tungstenite/blob/master/examples/echo-server.rs
use std::sync::{Arc, Mutex};
use std::{env, io::Error};
use std::fmt::Debug;
use std::time::Duration;
use futures_util::{StreamExt, SinkExt};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Millimeters(u64); // Max is 1.95 light years, should fit
#[derive(Serialize, Deserialize, Debug)]
struct Degrees(u16); // Modulo 360

// CraneState fits in 2*64 + 3*16 = 176 bits
#[derive(Serialize, Deserialize, Debug)]
struct CraneState {
    swing_rotation: Degrees,
    lift_elevation: Millimeters,
    elbow_rotation: Degrees,
    wrist_rotation: Degrees,
    gripper_open: Millimeters,
}


type Crane = Arc<Mutex<CraneState>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let crane = Arc::new(Mutex::new(CraneState {
        swing_rotation: Degrees(0),
        lift_elevation: Millimeters(0),
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
                        if msg.is_text() || msg.is_binary() {
                            write.send(msg.into()).await?;
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
