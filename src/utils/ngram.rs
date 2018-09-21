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

    /// Get a JSON serializable object from a ngram counter
    pub fn to_json(self)->JsonNgramCounter{
        JsonNgramCounter::new(self)
    }

    pub fn to_frequency(self)->NgramFrequency{
        NgramFrequency::new(self)
    }

    pub fn to_frequency_json(self)->JsonNgramFrequency{
        JsonNgramFrequency::new(self.to_frequency())
    }

    pub fn to_counterfrequency(self)->NgramCounterFrequency{
        NgramCounterFrequency::new(self)
    }

    pub fn to_counterfrequency_json(self)->JsonNgramCounterFrequency{
        JsonNgramCounterFrequency::new(self.to_counterfrequency())
    }

    pub fn ngram(&self)->usize{
        self.ngram
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonNgramCounter{
    ngram: usize,
    count: Vec<(Vec<u8>, usize)>,
    size: usize
}

impl JsonNgramCounter{
    pub fn new(counter: NgramCounter)->Self{
        let ngram = counter.ngram();
        let size = counter.size();
        let mut count = vec!();

        for (k, v) in counter.count_owned(){
            count.push((k, v));
        }

        Self{
            ngram: ngram,
            count: count,
            size: size
        }
    }
}


/// Frequency of each ngrams
pub struct NgramFrequency{
    ngram: usize,
    freq: HashMap<Vec<u8>, f64>
}

impl NgramFrequency{
    pub fn new(counter: NgramCounter)->Self{
        let ngram = counter.ngram();
        let fsize = counter.size() as f64;
        let mut freq = HashMap::new();

        for (k, v) in counter.count_owned(){
            freq.insert(k, v as f64 / fsize);
        }

        Self{
            ngram: ngram,
            freq: freq
        }
    }

    pub fn ngram(&self)->usize{
        self.ngram
    }

    pub fn freq(&self)->&HashMap<Vec<u8>, f64>{
        &self.freq
    }

    pub fn freq_owned(self)->HashMap<Vec<u8>, f64>{
        self.freq
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonNgramFrequency{
    ngram: usize,
    freq: Vec<(Vec<u8>, f64)>
}

impl JsonNgramFrequency{
    pub fn new(frequency: NgramFrequency)->Self{
        let ngram = frequency.ngram();
        let mut freq = vec!();

        for (k, v) in frequency.freq_owned(){
            freq.push((k, v));
        }

        Self{
            ngram: ngram,
            freq: freq
        }
    }
}


/// Frequency an number of each ngrams
pub struct NgramCounterFrequency{
    ngram: usize,
    count: HashMap<Vec<u8>, (usize, f64)>,
    size: usize
}

impl NgramCounterFrequency{
    pub fn new(counter: NgramCounter)->Self{
        let ngram = counter.ngram();
        let mut count = HashMap::new();
        let fsize = counter.size() as f64;
        let size = counter.size();

        for (k, v) in counter.count_owned(){
            count.insert(k, (v, v as f64 / fsize));
        }

        Self{
            ngram: ngram,
            count: count,
            size: size
        }
    }

    pub fn ngram(&self)->usize{
        self.ngram
    }

    pub fn count(&self)->&HashMap<Vec<u8>, (usize, f64)>{
        &self.count
    }

    pub fn count_owned(self)->HashMap<Vec<u8>, (usize, f64)>{
        self.count
    }

    pub fn size(&self)->usize{
        self.size
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonNgramCounterFrequency{
    ngram: usize,
    count: Vec<(Vec<u8>, usize, f64)>,
    size: usize
}

impl JsonNgramCounterFrequency{
    pub fn new(counter: NgramCounterFrequency)->Self{
        let ngram = counter.ngram();
        let size = counter.size();
        let mut count = vec!();

        for (k, v) in counter.count_owned(){
            count.push((k, v.0, v.1));
        }

        Self{
            ngram: ngram,
            count: count,
            size: size
        }
    }
}
