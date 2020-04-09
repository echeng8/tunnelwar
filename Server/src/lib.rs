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

    #[export]
    fn player_spawn_location(&self, _owner: Node) -> Vector2{
        Vector2::new(250.0, 250.0)
    }
}

fn init(handle: init::InitHandle ){
    handle.add_class::<Server>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();