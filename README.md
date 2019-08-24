# Bleep server

This is a demo for using Docker with a small service. The service is written in Rust and uses the Actix framework.

## Requirements
* Docker
* docker-compose

## Instructions
`cd` into the `micro_number` directory, then run:
```shell
docker-compose up
```
The container should be up and running when you see `binding successful` in the terminal.

## Endpoints
* `127.0.0.1/random/`: returns a random 8-bit unsigned number.
* `127.0.0.1/negative/`: return a random number from -255 to 0.
* `127.0.0.1/prime/<number>`: checks whether `<number>` (32-bit) is prime or not. 