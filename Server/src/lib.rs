#[macro_use] extern crate gdnative;
use gdnative::*;
use std::collections::HashMap;

mod chunk_data;
use chunk_data::ChunkData;

#[derive(NativeClass)]
#[inherit(Node)]
struct Server {
    players: HashMap<i64, String>,
    world: Vec<Vec<ChunkData>>
}

#[methods]
impl Server {
    fn _init(_owner: Node) -> Self {
        let world_size = 64;
        let mut chunk_row = Vec::with_capacity(world_size);

        let mut i = 0;
        while i != 64 {
            chunk_row.push(ChunkData::new());
            i += 1;
        }

        let mut world = Vec::with_capacity(world_size);

        i = 0;
        while i != 64 {
            world.push(chunk_row.clone());
            i += 1;
        }

        Self {
            players: HashMap::new(),
            world: world
        }
    }

    #[export]
    fn player_spawn_location(&self, _owner: Node) -> Vector2{
        Vector2::new(250.0, 250.0)
    }

    #[export]
    fn register_player(&mut self, owner: Node, id: i64, name: GodotString) {
        // Add him to our list
        self.players.insert(id, name.to_string());

        unsafe {
            let mut gamestate = owner.get_node(NodePath::from("/root/gamestate")).unwrap();

            // Add everyone to new player:
            for p_id in self.players.keys() {
                gamestate.rpc_id(id, GodotString::from("register_player"), &[Variant::from(*p_id), Variant::from(&self.players[p_id])]);
            }

            gamestate.rpc(GodotString::from("register_player"), &[Variant::from(id), Variant::from(&name)]); // Send new dude to all players
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

    #[export]
    fn update_chunks(&self, owner: Node) {
        let players_iter = self.players.keys();
        unsafe {
            for player_id in players_iter {
                // Get player node
                let player_node = owner.get_node(NodePath::new(&GodotString::from(player_id.to_string()))).unwrap().cast::<Node2D>().unwrap();
                
                // Find out which chunk the player is currently in
                let chunk_pos = Self::get_chunk_pos(player_node.get_position());

                let mut updating_chunk_pos = chunk_pos - Vector2::new(1.0, 1.0);

                loop {
                    let updating_chunk = &self.world[updating_chunk_pos.x as usize][updating_chunk_pos.y as usize];
                    // Check if chunk must be updated
                    let tunnel_option = updating_chunk.to_be_rendered(player_id);
                    if tunnel_option != None {
                        let tunnel = tunnel_option.unwrap();
                        let blocks = updating_chunk.get_tunnel_blocks(tunnel);
                        // Update the chunk
                        // TODO: Self::update_chunk(player_id, blocks); or just directly do an rpc
                    }

                    // Check if we're at the last chunk
                    if updating_chunk_pos - Vector2::new(1.0, 1.0) == chunk_pos {
                        break
                    }
                    // Check if chunk is at end of row
                    else if updating_chunk_pos.x == (chunk_pos.x + 1.0) {
                        updating_chunk_pos.x = chunk_pos.x - 1.0;
                        updating_chunk_pos.y -= 1.0;
                    } else {
                        updating_chunk_pos.x += 1.0;
                    }
                }
            }
        }
    }

    fn get_chunk_pos(position: Vector2) -> Vector2{
        Vector2::new(position.x % 8.0, position.y % 8.0)
    }

    /*#[export]
    fn get_player_name(&self, _owner: Node, id:i64) -> GodotString {
        GodotString::from(self.players.get(&id).unwrap())
    }*/
}

fn init(handle: init::InitHandle ){
    handle.add_class::<Server>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();