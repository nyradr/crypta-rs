use std::iter::Iterator;
use std::collections::HashMap;

/// Split the text in group of ngram characters
pub struct NGramSpliter<'a>{
    text: &'a[u8],
    ngram: usize,
    index: usize
}

impl<'a> NGramSpliter<'a>{

    /// Create a new ngram spliter for the given text
    pub fn new(text: &'a[u8], ngram: usize)->Self{
        Self{
            text: text,
            ngram: ngram,
            index: 0
        }
    }
}

impl<'a> Iterator for NGramSpliter<'a>{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item>{
        if self.index < self.text.len(){
            let ni = self.index + self.ngram;
            let sl = &self.text[self.index..ni];
            let mut elem = vec![];
            for e in sl{
                elem.push(e.clone());
            }

            self.index = ni;
            Some(elem)
        }else{
            None
        }
    }
}

/// Count the ngram occurence in a text
pub fn ngram_count(text: &[u8], ngram: usize)->HashMap<Vec<u8>, usize>{
    let spliter = NGramSpliter::new(text, ngram);
    let mut count = HashMap::new();

    for gram in spliter{
        *count.entry(gram).or_insert(0) += 1;
    }

    count
}

pub fn ngram_freqency(text: &[u8], ngram: usize)->HashMap<Vec<u8>, f64>{
    let spliter = NGramSpliter::new(text, ngram);
    let mut freq = HashMap::new();
    let inc: f64 = 1. / text.len() as f64;

    for gram in spliter{
        *freq.entry(gram).or_insert(0.) += inc;
    }

    freq
}
