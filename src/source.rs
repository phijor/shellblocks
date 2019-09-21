use crate::block::Block;

pub trait Source {
    fn get_block(&self) -> Option<Block>;
}
