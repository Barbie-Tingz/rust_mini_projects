/* The Agency Server

**The bit:** Every branch, every employee, every cursed object — it's all one system
now, and clients are submitting requests over the internet like it's a real business.

**Your mission:** Build a small multithreaded web server tying the earlier systems
together.*/

// 1. Set up a basic TCP listener using std::net::TcpListener bound to a local address and port.
// 2. Accept incoming connections in a loop using .incoming().
// 3. Read the incoming request from a connection stream and print it to the terminal.
// 4. Send back a simple hardcoded HTTP response (status line + a short themed body) over
//    the stream.
// 5. Move the connection-handling logic into its own function.
// 6. Build a simple thread pool (or adapt the one from the book) so multiple requests can
//    be handled at the same time instead of one at a time.
// 7. Route at least two different paths (e.g. "/" and "/cases") to different themed responses.
// 8. Test it by hitting the server from a browser or curl, and confirm both routes
//    respond correctly.