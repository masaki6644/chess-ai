use core::Position;

#[derive(Clone)]
pub struct PositionSample {
    pub pos: Position,
    pub ply: usize,
    pub total_plies: usize,
    
}