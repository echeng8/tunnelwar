#[macro_use] extern crate gdnative;
use gdnative::*;
use std::collections::HashMap;

mod chunk_data;
use chunk_data::ChunkData;

#[derive(NativeClass)]
#[inherit(Node)]
struct Server {
    players: HashMap<i64, String>,
    world: Vec<Vec<ChunkData>>,
    tunnels: Vec<usize>
}

static WORLD_SIZE: usize = 64;

#[methods]
impl Server {
    fn _init(_owner: Node) -> Self {
        let mut chunk_row = Vec::with_capacity(WORLD_SIZE);

        for i in 0..64 {
            if i == 0 || i == 63 {
                chunk_row.push(ChunkData::new(true));
            } else {
                chunk_row.push(ChunkData::new(false));
            }
        }

        // TODO add one row at start and one at end that is all border
        let mut border_chunk_row = Vec::with_capacity(WORLD_SIZE);
        for _i in 0..64 {
            border_chunk_row.push(ChunkData::new(true));
        }

        let mut world = Vec::with_capacity(WORLD_SIZE);

        world.push(border_chunk_row.clone());
        for _i in 0..62 {
            world.push(chunk_row.clone());
        }
        world.push(border_chunk_row);
        

        Self {
            players: HashMap::new(),
            world: world,
            tunnels: Vec::new()
        }
    }

    #[export]
    fn initialize_world(&mut self, owner: Node) {
        // for x in 6..12 {
        //     for y in 6..12 {
        //         self.remove_block(owner, Vector2::new(x as f32, y as f32));
        //     }
        // }

        // Initialize world map on server-side
        unsafe {
            let mut tile_map = owner.get_node(NodePath::from("/root/World/TileCollision/TileMap")).unwrap().cast::<TileMap>().unwrap();
            for chunk_x in 0..WORLD_SIZE {
                for chunk_y in 0..WORLD_SIZE {
                    let chunk = &self.world[chunk_x][chunk_y];
                    for x in 0..8 {
                        for y in 0..8 {
                            tile_map.set_cell(
                                x + (chunk_x as i64 * 8),
                                y + (chunk_y as i64 * 8),
                                chunk.blocks[x as usize][y as usize] - 1,
                                false, false, false,
                                Vector2::new(0.0, 0.0)
                            );
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

    // #[export]
    // fn is_peer_connected(&self, _owner: Node, id: i64) -> bool {
    //     self.players.contains_key(&id)
    // }

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
                
                godot_print!("{}", player_node.get_position());
                
                // Find out which chunk the player is currently in
                let block_pos = Self::get_block_pos(player_node.get_position());
                let chunk_pos = Self::get_chunk_pos(block_pos);
                // let player_chunk = &mut self.world[chunk_pos.x as usize][chunk_pos.y as usize];
                // let player_tunnel = player_chunk.get_tunnel_id(block_pos);
                // player_chunk.players.get_mut(&player_tunnel).unwrap().push(*player_id);

                for chunk_x in -1i64..2 {
                    for chunk_y in -1i64..2 {
                        let updating_chunk = &mut self.world[(chunk_pos.x as i64 + chunk_x) as usize][(chunk_pos.y as i64 + chunk_y) as usize];

                        let tunnel_option = updating_chunk.to_be_rendered(player_id, Self::get_block_pos(player_node.get_position()) - chunk_pos * 8.0);

                        if tunnel_option.is_none() {
                            continue
                        } else {
                            godot_print!("chunk loaded");
                            let tunnel_id = tunnel_option.unwrap();
                            let tunnel_blocks = updating_chunk.get_tunnel_blocks(tunnel_id);
                            updating_chunk.render_for(*player_id);

                            let mut world_node = owner.get_node(NodePath::from("/root/World")).unwrap();
                            let mut updating_block = Vector2::new(chunk_pos.x + chunk_x as f32, chunk_pos.y + chunk_y as f32) * 8.0;
                            //godot_print!("{}", updating_block);
                            for row in tunnel_blocks.iter() {
                                for block in row {
                                    // godot_print!("Updated block {} for {}", updating_block, player_node.get_position() / 64.0);
                                    world_node.rpc_id(
                                        *player_id, GodotString::from("set_cell"),
                                        &[
                                            Variant::from_i64(updating_block.x as i64),
                                            Variant::from_i64(updating_block.y as i64),
                                            Variant::from_i64(block - 1)
                                        ]
                                    );
                                    updating_block.x += 1.0;
                                }
                                updating_block.y += 1.0;
                                updating_block.x -= 8.0;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_block_pos(position: Vector2) -> Vector2 {
        Vector2::new((position.x / 64.0) as i32 as f32, (position.y / 64.0) as i32 as f32)
    }

    fn get_chunk_pos(block_pos: Vector2) -> Vector2 {
        Vector2::new((block_pos.x / 8.0) as i32 as f32, (block_pos.y / 8.0) as i32 as f32)
    }

    #[export]
    fn is_block_breakable(&self, _owner: Node, block_pos: Vector2) -> bool {
        let chunk_pos = Self::get_chunk_pos(block_pos);
        if chunk_pos.x == 0. || chunk_pos.x == 63.0 || chunk_pos.y == 0.0 || chunk_pos.y == 63.0 {
            false
        } else { true }
    }

    #[export]
    fn remove_block(&mut self, _owner: Node, block_pos: Vector2) {
        // Remove block from map storage
        let chunk_pos = Self::get_chunk_pos(block_pos);
        let chunk_block_pos = Vector2::new(block_pos.x % 8.0, block_pos.y % 8.0);
        let mut chunk = self.world[chunk_pos.x as usize][chunk_pos.y as usize].clone();
        chunk.blocks[chunk_block_pos.x as usize][chunk_block_pos.y as usize] = 0;
        self.world[chunk_pos.x as usize][chunk_pos.y as usize] = chunk.clone();

        // Update tunnels
        let mut resulting_tunnel_id = 0;
        for x in -1i64..2 {
            for y in -1i64..2 {
                // TODO: Get tunnel id of neighboring block
                // let neighbor_tunnel_id;
                // if x < 0 || y < 0 {
                //     let chunk_pos = Self::get_chunk_pos(Vector2::new(block_pos.x + x as f32, block_pos.y + y as f32));
                //     let chunk_block_pos = Vector2::new(block_pos.x + x as f32 % 8.0, block_pos.y + x as f32 % 8.0);
                //     let chunk = &mut self.world[chunk_pos.x as usize][chunk_pos.y as usize];
                //     neighbor_tunnel_id = chunk.get_tunnel_id(Vector2::new(chunk_block_pos.x + x as f32, chunk_block_pos.y + y as f32));
                // } else {
                //     neighbor_tunnel_id = chunk.get_tunnel_id(Vector2::new(chunk_block_pos.x + x as f32, chunk_block_pos.y + y as f32));
                // }
                // TODO: account for neighbors outside of current chunk
                let neighbor_tunnel_id = 0;

                // A resulting tunnel has not already been found
                if resulting_tunnel_id != 0 && neighbor_tunnel_id != resulting_tunnel_id {
                    self.merge_tunnels(resulting_tunnel_id, neighbor_tunnel_id);
                } else if neighbor_tunnel_id != 0 {
                    // Does not set resulting tunnel if neighbor tunnel is 0 as it
                    // already has the value 0, so it would be an extra unnecessary operation
                    resulting_tunnel_id = neighbor_tunnel_id;
                }
            }
        }

        // If there were no tunnels along
        if resulting_tunnel_id == 0 {
            // Create new tunnel
            let mut new_tunnel_id = 1;
            for i in 0..self.tunnels.len() {
                if self.tunnels[i] != i {
                    new_tunnel_id = self.tunnels[i-1] + 1;
                    // self.tunnels;
                }
            }
            // Add tunnel to chunk.players
            chunk.players.insert(new_tunnel_id as i64, Vec::new());
        }

        // Make chunk unrendered for all players in tunnel of block
        chunk.unrender(resulting_tunnel_id);

        // Update tunnel_positions
        chunk.tunnel_positions[chunk_block_pos.x as usize][chunk_block_pos.y as usize] = resulting_tunnel_id;
        self.world[chunk_pos.x as usize][chunk_pos.y as usize] = chunk;
    }

    fn merge_tunnels(&self, tunnel1_id: i64, tunnel2_id:i64) {
        // TODO
    }
}

fn init(handle: init::InitHandle ){
    handle.add_class::<Server>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();