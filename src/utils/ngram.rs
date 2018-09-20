use std::io::Read;
use std::iter::Iterator;
use std::collections::HashMap;

use utils::bytesreader::BytesReader;

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

    pub fn ngram(&self)->usize{
        self.ngram
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

/// Keep tracks of the number of occurences of each ngrams in a text
pub struct NgramCounter{
    ngram: usize,
    count: HashMap<Vec<u8>, usize>,
    size: usize
}

impl NgramCounter{
    /// Create a new empty NGramCounter
    pub fn new(ngram: usize)->Self{
        Self{
            ngram: ngram,
            count: HashMap::new(),
            size: 0
        }
    }

    /// Create a new NgramCounter from a spliter
    pub fn from_spliter<R: Read>(spliter: NGramSpliter<R>)->Self{
        let mut counter = Self::new(spliter.ngram());
        counter.append(spliter);
        counter
    }

    /// Create a new NgramCounter from a reader
    pub fn from_read<R: Read>(reader: R, ngram: usize)->Self{
        let spliter = NGramSpliter::new(reader, ngram);
        Self::from_spliter(spliter)
    }

    /// Create a new NgramCounter from a slice of bytes
    pub fn from_bytes(bytes: &[u8], ngram: usize)->Self{
        let br = BytesReader::new(bytes);
        Self::from_read(br, ngram)
    }

    /// Count the number of occurences of each ngrams from a spliter.
    /// The spliter must have the same ngram as the counter.
    pub fn append<R: Read>(&mut self, spliter: NGramSpliter<R>){
        if spliter.ngram() == self.ngram{
            for gram in spliter{
                *self.count.entry(gram).or_insert(0) += 1;
                self.size += 1;
            }
        }
    }

    /// Count the number of occurences of each ngrams from a reader.
    pub fn append_read<R: Read>(&mut self, reader: R){
        let spliter = NGramSpliter::new(reader, self.ngram);
        self.append(spliter)
    }

    /// Count the number of occurences of each ngrams from a slice of bytes.
    pub fn append_bytes(&mut self, bytes: &[u8]){
        let br = BytesReader::new(bytes);
        self.append_read(br)
    }

    /// Get the ngram count
    pub fn count(&self)->&HashMap<Vec<u8>, usize>{
        &self.count
    }

    /// Get the ngram count as a owned hashmap
    pub fn count_owned(self)->HashMap<Vec<u8>, usize>{
        self.count
    }

    /// Get the total number of ngrams
    pub fn size(&self)->usize{
        self.size
    }

    /// Get the ngram counter as the frequency of each ngrams
    pub fn as_frequency(self)->HashMap<Vec<u8>, f64>{
        let mut freq = HashMap::new();
        let size = self.size as f64;

        for (k, v) in self.count{
            freq.insert(k, v as f64 / size );
        }

        freq
    }
}

/// Clap validator testing if a string can be a valid ngram value.
pub fn cli_validator_ngram(v: String)->Result<(), String>{
    match usize::from_str_radix(&v, 10){
        Ok(_) => Ok(()),
        Err(s) => Err(s.to_string())
    }
}
