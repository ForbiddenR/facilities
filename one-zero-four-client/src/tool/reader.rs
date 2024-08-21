pub struct Reader<'a> {
    index: usize,
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader { index: 0, data }
    }

    pub fn read(&mut self, length: usize) -> Option<&[u8]> {
        if self.index >= self.data.len() {
            None
        } else if length >= self.data.len() - self.index {
            let result = &self.data[self.index..self.data.len()];
            self.index = self.data.len();
            Some(result)
        } else {
            let result = &self.data[self.index..self.index + length];
            self.index = self.index + length;
            Some(result)
        }
    }

    pub fn unsafe_read(&mut self, length: usize) -> &[u8] {
        if length >= self.data.len() - self.index {
            let result = &self.data[self.index..self.data.len()];
            self.index = self.data.len();
            result
        } else {
            let result = &self.data[self.index..self.index + length];
            self.index = self.index + length;
            result
        }
    }
}
