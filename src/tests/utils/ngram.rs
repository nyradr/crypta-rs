use std::collections::HashMap;

use utils::ngram::*;
use utils::bytesreader::BytesReader;

#[test]
fn test_spliter_1(){
    let text = [1, 1, 2, 2, 3, 3, 4, 4];
    let br = BytesReader::new(&text);
    let mut i = 0;

    let spliter = NGramSpliter::new(br, 1);

    for gram in spliter{
        let tg = vec![text[i].clone()];
        assert_eq!(gram, tg);
        i += 1;
    }
}

#[test]
fn test_spliter_2(){
    let text = [1, 1, 2, 2, 3, 3, 4, 4];
    let br = BytesReader::new(&text);
    let mut i = 0;

    let spliter = NGramSpliter::new(br, 2);

    for gram in spliter{
        let tg = vec![text[i], text[i+1]];
        assert_eq!(gram, tg);
        i += 2;
    }
}

#[test]
fn test_spliter_3(){
    let text = [1, 1, 2, 2, 3, 3, 4, 4];
    let br = BytesReader::new(&text);
    let mut i = 0;

    let spliter = NGramSpliter::new(br, 3);

    for gram in spliter{
        let tg = if i +2 < 8{
            vec![text[i], text[i+1], text[i+2]]
        }else{
            vec![text[i], text[i+1], 0]
        };

        assert_eq!(gram, tg);
        i += 3;
    }
}

#[test]
fn test_spliter_4(){
    let text = [];
    let br = BytesReader::new(&text);

    let mut spliter = NGramSpliter::new(br, 2);
    assert_eq!(spliter.next(), None);
}


#[test]
fn test_ngramcounter_1() {
    let text = [1, 1, 2, 2, 2, 3];
    let counter = NgramCounter::from_bytes(&text, 1);

    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1], 2);
    count_oracle.insert(vec![2], 3);
    count_oracle.insert(vec![3], 1);

    assert_eq!(counter.size(), 6);
    assert_eq!(counter.count(), &count_oracle);
}

#[test]
fn test_ngramcounter_2() {
    let text = [1, 1, 2, 2, 2, 2];
    let counter = NgramCounter::from_bytes(&text, 2);

    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1, 1], 1);
    count_oracle.insert(vec![2, 2], 2);

    assert_eq!(counter.size(), 3);
    assert_eq!(counter.count(), &count_oracle);
}

#[test]
fn test_ngramcounter_3() {
    let text = [1, 1, 1, 2, 2];
    let counter = NgramCounter::from_bytes(&text, 3);

    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1, 1, 1], 1);
    count_oracle.insert(vec![2, 2, 0], 1);

    assert_eq!(counter.size(), 2);
    assert_eq!(counter.count(), &count_oracle);
}

#[test]
fn test_ngramcounter_4() {
    let text = [];
    let counter = NgramCounter::from_bytes(&text, 2);

    let count_oracle = HashMap::new();

    assert_eq!(counter.size(), 0);
    assert_eq!(counter.count(), &count_oracle);
}

#[test]
fn test_ngramcounter_5() {
    let text = [1, 1, 1, 2, 2];
    let mut counter = NgramCounter::from_bytes(&text, 3);
    counter.append_bytes(&text);

    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1, 1, 1], 2);
    count_oracle.insert(vec![2, 2, 0], 2);

    assert_eq!(counter.size(), 4);
    assert_eq!(counter.count(), &count_oracle);
}
