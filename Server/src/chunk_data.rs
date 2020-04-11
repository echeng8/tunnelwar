use gdnative::*;

//#[derive(Copy)]
pub struct ChunkData {
    players: bool/* TODO */,
    blocks: [[i64; 8]; 8],
    rendered_for: Vec<i64>
}

impl Clone for ChunkData {
    fn clone(&self) -> Self {
        ChunkData {
            players: self.players,
            blocks: self.blocks,
            rendered_for: self.rendered_for.clone()
        }
    }
}

impl ChunkData {
    pub fn new() -> Self{
        Self {
            players: false,
            blocks: [[1; 8]; 8],
            rendered_for: vec![]
        }
    }

    pub fn is_rendered_for(&self, id: &i64) -> bool {
        self.rendered_for.contains(id)
    }
}