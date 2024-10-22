[![progress-banner](https://backend.codecrafters.io/progress/http-server/df3a5ba5-8a56-4064-bc75-b926f75ca448)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is my solution to the
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

In this repository you'll find a HTTP/1.1 server
that is capable of serving multiple clients. 

This server can
- Work with multiple clients at the same time using threads
- Save env context using command line args
- Handle routes that work with files
- Be extended by using the abstraction layer for creating new routes

# How to Run

1. Ensure you have `cargo (1.80)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. 
   - Note: if you want to work with files specify a directory flag 
   `./your_program.sh --directory <your_directory>`

To learn more about TCP servers,
[HTTP request syntax](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

