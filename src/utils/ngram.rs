use std::io::Read;
use std::iter::Iterator;

/// Split the text in group of ngram characters
pub struct NGramSpliter<T: Read>{
    reader: T,
    ngram: usize,
    buf: Vec<u8>,
    padding: u8
}

impl<T: Read> NGramSpliter<T>{

    /// Create a new ngram spliter for the given text
    pub fn new(reader: T, ngram: usize)->Self{
        Self::with_padding(reader, ngram, 0)
    }

    /// Create a new ngram spliter for the given text. Set the padding byte used if the text length is not a multiple of ngram
    pub fn with_padding(reader: T, ngram: usize, padding: u8)->Self{
        let mut buf = Vec::with_capacity(ngram);
        for _ in 0..ngram{
            buf.push(0);
        }

        Self{
            reader: reader,
            ngram: ngram,
            buf: buf,
            padding: padding
        }
    }
}

impl<T: Read> Iterator for NGramSpliter<T>{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item>{
        match self.reader.read(&mut self.buf){
            Ok(0) => None,
            Ok(mut s) => {
                while s < self.ngram{ // add padding to shorter ngram
                    self.buf[s] = self.padding;
                    s += 1;
                }

                Some(self.buf.clone())
            },
            _ => None
        }
    }
}