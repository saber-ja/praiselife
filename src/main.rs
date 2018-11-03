extern crate gotham;

use gotham::state::State;

const HELLO_WORLD: &'static str = "Hello World!";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}