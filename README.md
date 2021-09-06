# DNS resolver implemented with Rust

## Purpose for this repository
- Studying Rust
- I wanted to get some experience implementing based on RFCs.

## How to use
1. run the command `cargo run`, start UDP server on port `2053`.
2. run the command `dig -p 2053 @127.0.0.1 +noedns google.com`, return answer.

## Ref
- [RFC1035](https://datatracker.ietf.org/doc/html/rfc1035)
