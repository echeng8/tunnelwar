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

fn init(handle: init::InitHandle ){
    handle.add_class::<Server>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();