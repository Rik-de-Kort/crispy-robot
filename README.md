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