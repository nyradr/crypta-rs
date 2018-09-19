use std::io::{Read, Result};

/// Read a slice of bytes with the Read interface
pub struct BytesReader<'a>{
    bytes: &'a [u8],
    index: usize
}

impl<'a> BytesReader<'a>{
    pub fn new(bytes: &'a [u8])->Self{
        Self{
            bytes: bytes,
            index: 0
        }
    }
}

impl<'a> Read for BytesReader<'a>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>{
        let mut i = 0;

        while i < buf.len() && self.index < self.bytes.len(){
            buf[i] = self.bytes[self.index];
            i += 1;
            self.index += 1;
        };


        Ok(i)
    }
}