use gdnative::*;
use std::collections::HashMap;

//#[derive(Copy)]
pub struct ChunkData {
    pub players: HashMap<i64, Vec<i64>>/* TODO */,
    pub blocks: [[i64; 8]; 8],
    pub tunnel_positions: [[i64; 8]; 8],
    rendered_for: Vec<i64>
}

impl Clone for ChunkData {
    fn clone(&self) -> Self {
        ChunkData {
            players: self.players.clone(),
            blocks: self.blocks,
            tunnel_positions: self.tunnel_positions,
            rendered_for: self.rendered_for.clone()
        }
    }
}

impl ChunkData {
    pub fn new(is_border: bool) -> Self {
        if is_border {
            Self {
                players: HashMap::new(),
                blocks: [[3; 8]; 8],
                tunnel_positions: [[0; 8]; 8],
                rendered_for: vec![]
            }
        } else {
            Self {
                players: HashMap::new(),
                blocks: [[1; 8]; 8],
                tunnel_positions: [[0; 8]; 8],
                rendered_for: vec![]
            }
        }
    }

    pub fn get_tunnel_id(&self, block_pos: Vector2) -> i64 {
        self.tunnel_positions[block_pos.x as usize][block_pos.y as usize]
    }

    pub fn to_be_rendered(&self, player_id: &i64, block_pos: Vector2) -> Option<i64> {
        // TODO: switch over to using the players field
        // for figuring out a player's tunnel's id
        if self.rendered_for.contains(player_id) {
            None
        } else {
            Some(self.get_tunnel_id(block_pos))
        }
    }

    pub fn render_for(&mut self, id: i64) {
        self.rendered_for.push(id);
    }

    pub fn unrender(&mut self, tunnel_id: i64) {
        // TODO: unrender for neighbouring chunks as well
        // for player in &self.players[&tunnel_id] {
        //     if self.rendered_for.contains(player) {
        //         let index = self.rendered_for.binary_search(player).unwrap();
        //         self.rendered_for.remove(index);
        //     }
        // }

        self.rendered_for.clear();
    }

    pub fn get_tunnel_blocks(&self, tunnel_id: i64) -> [[i64; 8]; 8] {
        let mut tunnel_blocks = [[1; 8]; 8];

        for x in 0..8 {
            for y in 0..8 {
                if self.tunnel_positions[x][y] == tunnel_id {
                    tunnel_blocks[x][y] = self.blocks[x][y];
                }
            }
        }

        tunnel_blocks
    }
}