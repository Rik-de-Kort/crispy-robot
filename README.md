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
My standard approach to a greenfield system is to write straightforward code that does straightforward things with a minimum of indirection.  
I try to always keep something "working", something that you can see. If you look at the commit history, that's basically the story they tell.
I clean up as I go along, and most abstractions fall out that way.

A good example is the `CraneModel` in the frontend, where I first wrote all the code to set up, position, and render the crane inline in the component's `onMount`, and then later factored that out by separating out the set up and position process into a separate class. This makes it very obvious where to change things about the crane like its dimensions or joints.

The `CraneState` abstraction in the backend is a little more premeditated. It's mutable state, so best to keep the mutating to one spot, preferably one method. There's a clear distinction between what the frontend is *telling* the crane to do, and what the crane itself is *doing*. This makes it easy to localize any issues when the crane doesn't move like you expect.

## Backend
### Overview
The main work is done on a struct `CraneState` which can execute a `CraneCommand` and updates its state.

### Libraries
On Rust side, I set up a Tokio server with Tokio-Tungstenite to handle websockets. This is the most commonly recommended setup for websockets in Rust, documentation is good and there are plenty of examples. It does force me to use Async Rust which I'm a little less happy about, but then again in a real life situation you might want to have multiple clients, so it's not an unthinkable combination.

### Types
I decided to use newtypes to make sure that I wouldn't mix rotation and distance measures. On the frontend I didn't because Typescript doesn't support that very well. The safety benefit didn't weigh up against the verbosity cost in this project, I would probably not bother next time.

### Tests
It's much harder to make a mistake if you can see the thing working. Apply unit testing mostly as a safety net in case an implementation bug wouldn't be immediately obvious.

## Frontend
### Libraries
The main reason for choosing Svelte on the frontend is that I'm familiar with it and I like building with it. I'm already picking up a new thing (Tokio) on the backend, and I would like to actually get to building. Since I would like to keep the project footprint small, I opted for a barebones Vite project rather than pulling in SvelteKit.

### Types
Typescript doesn't support newtypes very well, so I went with just bare `number`s.

### Immediate mode vs Retained Mode
The assignment states that the frontend will play a small animation based on the commands in the backend.
I opted instead for an immediate-mode presentation layer in the frontend. The only thing the frontend does is turn a `CraneState` into a picture, like a React component.
Websocket latency is pretty low, especially with a local setup, so I think this works well in this use case.
In return we don't have to write any animation code. If animations were necessary, I would choose to interpolate the current state and the desired state.
Introducing an animation command means that we are shipping diffs over the wire, which means we *have* to rely on synchronization, because to know the desired state we need to take the initial state and apply all the packages. It's much easier to send the desired state; if the connection is interrupted, the next packet has the desired state in one go.

More on this idea in the context of graphics programming [here](https://caseymuratori.com/blog_0001).

# Overall impression
Completing the assignment to this state of doneness felt pretty straightforward. The biggest hurdle for me was just the unfamiliarity of the tools. 

I spent a lot of time on working out the movement of the crane and the 3D positions of the different components. Last time I did anything with 3D modelling was in 2021, so that stands to reason. There's probably also more idiomatic ways to do the things I did, but I'd never used Three.js or WebGL before.

I didn't take the time to implement the 3d coordinate input, but here's how to derive it.

* The lift position is the only thing that can change the y-coordinate constraint. The gripper is at a fixed offset.
* In the xz-plane, convert coordinates to polar. Derive the correct elbow rotation from the radius (arm lengths are fixed, so this is just trigonometry). The correct swing rotation is the polar angle minus a small offset (`(pi - elbow_rotation) / 2`) to account for the arm angle.

There are a lot of things I could take as next steps with this project. There are some todos sprinkled throughout the project, there's no handling of partial messages, etc.
The project also doesn't take into account any bounds: it just snaps the crane to where you tell it to go. Implementing this straightforwardly goes into `execute` on `CraneState`.

Most of these steps are quite straightforward, they just take more work. I think that's more or less ideal.