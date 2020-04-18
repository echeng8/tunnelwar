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

static WORLD_SIZE: usize = 64;

#[methods]
impl Server {
    fn _init(_owner: Node) -> Self {
        let mut chunk_row = Vec::with_capacity(WORLD_SIZE);

        let mut i = 0;
        while i != 64 {
            chunk_row.push(ChunkData::new());
            i += 1;
        }

        let mut world = Vec::with_capacity(WORLD_SIZE);

        i = 0;
        while i != 64 {
            world.push(chunk_row.clone());
            i += 1;
        }

        let mut start_chunk = world[0][0].blocks;
        start_chunk[3][3] = 0;
        start_chunk[3][4] = 0;
        start_chunk[4][4] = 0;
        start_chunk[4][3] = 0;
        world[0][0].blocks = start_chunk;
        

        Self {
            players: HashMap::new(),
            world: world
        }
    }

    #[export]
    fn initialize_world(&self, owner: Node) {
        // Initialize world map on server-side
        unsafe {
            let mut tile_map = owner.get_node(NodePath::from("/root/World/TileCollision/TileMap")).unwrap().cast::<TileMap>().unwrap();
            for chunk_x in 0..WORLD_SIZE {
                for chunk_y in 0..WORLD_SIZE {
                    let chunk = &self.world[chunk_x][chunk_y];
                    for x in 0..8 {
                        for y in 0..8 {
                            tile_map.set_cell(x + (chunk_x as i64 * 8), y + (chunk_y as i64 * 8), chunk.blocks[x as usize][y as usize] - 1, false, false, false, Vector2::new(0.0, 0.0));
                        }
                    }
                }
            }
        }
    }

    #[export]
    fn player_spawn_location(&self, _owner: Node) -> Vector2{
        Vector2::new(250.0, 250.0)
    }

    #[export]
    fn register_player(&mut self, _owner: Node, id: i64, name: GodotString) {
        // Add him to our list
        self.players.insert(id, name.to_string());
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
    fn update_chunks(&mut self, owner: Node) {
        let players_iter = self.players.keys().clone();
        unsafe {
            for player_id in players_iter {
                // Get player node
                let player_node_option = owner.get_node_or_null(NodePath::new(&GodotString::from(format!("/root/World/Players/{}", player_id.to_string()))));

                let player_node: Node2D;
                if !player_node_option.is_none() {
                    player_node = player_node_option.unwrap().cast::<Node2D>().unwrap();
                } else {
                    continue
                }          
                
                // Find out which chunk the player is currently in
                let chunk_pos = Self::get_chunk_pos(player_node.get_position());

                for chunk_x in -1i64..1 {
                    for chunk_y in -1i64..1 {
                        //godot_print!("{}", updating_chunk_pos);

                        let updating_chunk = &mut self.world[(chunk_pos.x as i64 + chunk_x) as usize][(chunk_pos.y as i64 + chunk_y) as usize];
                        // Check if chunk must be updated
                        let tunnel_option = updating_chunk.to_be_rendered(player_id, player_node.get_position());
                        if tunnel_option != None {
                            updating_chunk.render_for(*player_id);
                            let tunnel = tunnel_option.unwrap();
                            let blocks = updating_chunk.get_tunnel_blocks(tunnel);
                            // Update the chunk
                            let mut world_node = owner.get_node(NodePath::from("/root/World")).unwrap();
                            for x in 0..8 {
                                for y in 0..8 {
                                    world_node.rpc_id(*player_id, GodotString::from("set_cell"), &[Variant::from_i64(x + (chunk_pos.x as i64)), Variant::from_i64(y + (chunk_pos.y as i64)), Variant::from_i64(blocks[x as usize][y as usize] - 1)]);
                                }
                            }
                        }
                    }

                    // // Check if we're at the last chunk
                    // if updating_chunk_pos == chunk_pos { godot_print!("hi") }
                    // if updating_chunk_pos - Vector2::new(1.0, -1.0) == chunk_pos {
                    //     break
                    // }
                    // // Check if chunk is at end of row
                    // else if updating_chunk_pos.x == (chunk_pos.x + 1.0) {
                    //     updating_chunk_pos.x = chunk_pos.x - 1.0;
                    //     updating_chunk_pos.y -= 1.0;
                    // } else {
                    //     updating_chunk_pos.x += 1.0;
                    // }
                }
            }
        }
    }

    fn get_chunk_pos(position: Vector2) -> Vector2 {
        //godot_print!("{}", position);
        godot_print!("{}, {}", (position.x / 64.0) as i64 , (position.y / 64.0) as i64);
        Vector2::new((position.x / 64.0) as i32 as f32, (position.y / 64.0) as i32 as f32)
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