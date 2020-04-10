#[macro_use] extern crate gdnative;
use gdnative::*;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node)]
struct Server {
    players: HashMap<i64, String>
}

#[methods]
impl Server {
    fn _init(_owner: Node) -> Self {
        Self {
            players: HashMap::new()
        }
    }

    #[export]
    fn player_spawn_location(&self, _owner: Node) -> Vector2{
        Vector2::new(250.0, 250.0)
    }

    #[export]
    fn register_player(&mut self, mut owner: Node, id: i64, name: GodotString) {
        // Add him to our list
        self.players.insert(id, name.to_string());

        unsafe {
            // Add everyone to new player:
            for p_id in self.players.keys() {
                owner.rpc_id(id, GodotString::from("register_player"), &[Variant::from(*p_id), Variant::from(&self.players[p_id])]);
            }

            owner.rpc(GodotString::from("register_player"), &[Variant::from(id), Variant::from(&name)]); // Send new dude to all players
        }

        godot_print!("Client {} registered as {:?}", id, name)
    }

    #[export]
    fn unregister_player(&mut self, _owner: Node, id: i64) {
        self.players.remove(&id);
    }

    #[export]
    fn is_peer_connected(&self, _owner: Node, id: i64) -> bool {
        self.players.contains_key(&id)
    }
}

fn init(handle: init::InitHandle ){
    handle.add_class::<Server>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();