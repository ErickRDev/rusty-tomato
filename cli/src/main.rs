/// POC Definition
/// --------------
/// A CLI to bootstrap the pomodoro service as a daemon and to query the daemon with something trivial; like the current time.

/// MVP Definition
/// --------------
/// A CLI that can be used to manage (bootstrap, stop, etc) the daemon, query it and issue commands (start, stop, reset, etc).
/// A server that receives and stores pomodoro events from clients.
/// A database that the server can use to store pomodoro events.

fn main() {
    println!("Hello, world!");
}
