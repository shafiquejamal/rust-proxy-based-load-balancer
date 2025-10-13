# Introduction

This is a demo of a Rust proxy-based load balancer. 

# Requirements

- Podman (or you could use Docker, but you would have to make a small change to the `start-containers.sh` script)
- Rust (cargo)
- in the root of the project, add a `.env` file that looks like this:
```
HEALTH_CHECK_DELAY_MS_WORKER_000=10
HEALTH_CHECK_DELAY_MS_WORKER_001=11
HEALTH_CHECK_DELAY_MS_WORKER_002=12
WORKER_000_PORT=3000
WORKER_001_PORT=3001
WORKER_002_PORT=3002
```

(Any recent versions should work)

# How to use

- clone this repo
- in a terminal session, run `./start-only-worker-containers.sh` to start the backend containers
- in another terminal session, cd into the `load-balancer` directory and run `cargo run`
- open a web browser and navigate to `http://localhost:1337/health-check`. Refresh this a few times to see the effects of round-robin load balancing
