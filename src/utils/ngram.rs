use std::iter::Iterator;

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
