#[derive(Debug)]
pub struct Memory(Box<[u16; 2_usize.pow(16)]>);

impl Default for Memory {
    fn default() -> Self {
        Self(Box::new([0u16; 2_usize.pow(16)]))
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u16;

    fn index(&self, idx: u16) -> &Self::Output {
        &self.0[idx as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, idx: u16) -> &mut Self::Output {
        &mut self.0[idx as usize]
    }
}
