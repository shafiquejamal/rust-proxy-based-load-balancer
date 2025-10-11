# Introduction

This is a demo of a Rust proxy-based load balancer. 

# Requirements

- Podman (or you could use Docker, but you would have to make a small change to the `start-containers.sh` script)
- Rust (cargo)

(Any recent versions should work)

# How to use

- clone this repo
- in a terminal session, run `./start-containers.sh` to start the backend containers
- in another terminal session, cd into the `load-balancer` directory and run `cargo run`
- open a web browser and navigate to `http://localhost:1337/health-check`. Refresh this a few times to see the effects of round-robin load balancing
