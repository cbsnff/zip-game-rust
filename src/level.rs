pub type Cell = (i16, i16);

#[derive(Clone, Copy)]
pub struct Checkpoint {
    pub index: u8,
    pub cell: Cell,
}

#[derive(Clone)]
pub struct Level {
    pub size: i16,
    pub checkpoints: Vec<Checkpoint>,
}
