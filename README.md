# My Ping Server

## Description

`my_ping_server` is a robust ping server built using TCP sockets in Rust. This server can handle multiple connections simultaneously and responds to "ping" requests with the current time in EPOCH format.

## What is Ping?

Ping is a computer network administration utility used to test the reachability of a host on an IP network. It measures the round-trip time for messages sent from the originating host to a destination computer and back. The term comes from active sonar terminology, where a pulse of sound is sent and the echo is listened for to detect objects underwater.

Ping operates by sending ICMP echo request packets to the target host and waiting for an ICMP echo reply. The program reports errors, packet loss, and provides a statistical summary of the results, including minimum, maximum, mean round-trip times, and standard deviation of the mean.

## Project Structure

This project uses the following Rust libraries:

- [Tokio](https://tokio.rs/): For asynchronous I/O
- [structopt](https://docs.rs/structopt/): For command-line argument parsing
- [Bytes](https://docs.rs/bytes/): For handling byte sequences
- [Chrono](https://docs.rs/chrono/): For date and time manipulation

## Features

- Accepts multiple TCP connections simultaneously.
- Responds to "ping" requests with "pong TIME", where TIME is the current time in EPOCH format.

## Usage

### Server

To run the server, use the following command:

`cargo run --bin server -- --host <HOST> --port <PORT>`

Default values:

- HOST: 127.0.0.1
- PORT: 42422

### Example

#### Start the server:

`cargo run --bin server`

#### Connect to the server using `nc` (netcat):

`nc localhost 42422`

#### Send a "ping" request:

`ping`

#### The server will respond with:

`pong <EPOCH_TIME>`

#### You can also ping a specific URL:

`ping [url]`

### Bonus: `my_ping` Client

A bonus feature is provided to create a `my_ping` client that sends TCP echo requests to a specified host.

#### To run the client, use the following command:

`cargo run --bin my_ping -- <HOST>`

### Example

#### Run the client to ping amazon.com:

`cargo run --bin my_ping -- amazon.com`
