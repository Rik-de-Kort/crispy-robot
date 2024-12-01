# How to run
## Backend
```shell
cd backend
cargo build
cargo run
```

## Frontend
```shell
cd frontend
npm install
npm run dev
```

Point your browser to whatever it says, usually https://localhost:5173/

# Architectural Decisions
## Backend Libraries
On Rust side, I set up a Tokio server with Tokio-Tungstenite to handle websockets. This is the most commonly recommended setup for websockets in Rust, documentation is good and there are plenty of examples. It does force me to use Async Rust which I'm a little less happy about, but then again in a real life situation you might want to have multiple clients, so it's not an unthinkable combination.

## Frontend Libraries
The main reason for choosing Svelte on the frontend is that I'm familiar with it and I like building with it. I'm already picking up a new thing (Tokio) on the backend, and I would like to actually get to building. Since I would like to keep the project footprint small, I opted for a barebones Vite project rather than pulling in SvelteKit.

## Immediate mode vs Retained Mode
The assignment states that the frontend will play a small animation based on the commands in the backend.
I opted instead for an immediate-mode presentation layer in the frontend. The only thing the frontend does is turn a `CraneState` into a picture, like a React component.
Websocket latency is pretty low, especially with a local setup, so I think this works well in this use case.
In return we don't have to write any animation code. If we did, I would interpolate the current state and the desired state.

# Overall impression
Fairly straightforward process. I spent a lot of time getting the Three.js setup working right and working out the right formulas for positioning the various objects.  
The backend setup was very easy in comparison, no real trouble there. The impls for the new types were a bit of a hassle.
Some next steps I would (and might) take on this project:

* Move out Crane types into a separate file in the backend.
* Make sure the Move commands can't overrun their bounds. Actually take into account maximum speeds.
* Provide the coordinate input field.
* Don't load a model before a connection to the backend has been established. This would avoid the model popping in and out when you load the page the first time.
* Try and tie the front- and backend types together. This is the main source of duplication in the codebase right now. In my current role I write a fair amount of hacky generators for this sort of stuff.
* More ergonomic way of working with the CommandButtons, preferably being able to modify the CommandButtons based on the type they of command they are for.
* See if there's a more idiomatic way to handle errors and messages over a websocket. With normal sockets, you would have a buffer and save partial messages there, so you don't lose anything. Locally there's not much that does go wrong, but that'd be different in a deployment.
* Clean up the 3D model part. Make the gripper look like a gripper, have a light in the scene somewhere so you can see things better, that sort of thing.
