use gdnative::*;
use std::collections::HashMap;

//#[derive(Copy)]
pub struct ChunkData {
    players: HashMap<i64, Vec<i64>>/* TODO */,
    blocks: [[i64; 8]; 8],
    tunnel_positions: [[i64; 8]; 8],
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
    pub fn new() -> Self{
        Self {
            players: HashMap::new(),
            blocks: [[1; 8]; 8],
            tunnel_positions: [[1; 8]; 8],
            rendered_for: vec![]
        }
    }

    pub fn to_be_rendered(&self, player_id: &i64) -> Option<i64> {
        self.rendered_for.contains(player_id);
        None
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