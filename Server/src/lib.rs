#[macro_use] extern crate gdnative;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct Server;

#[methods]
impl Server {
    fn _init(_owner: Node) -> Self {
        Self
    }
}