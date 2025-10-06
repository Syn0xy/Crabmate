#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChessId {
    pub(crate) index: u32,
}

impl ChessId {
    pub const fn index(&self) -> u32 {
        self.index
    }
}
