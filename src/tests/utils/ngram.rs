use utils::ngram::*;
use std::collections::HashMap;

#[test]
fn test_spliter_1(){
    let text = [1, 1, 2, 2, 3, 3, 4, 4];
    let mut i = 0;

    let spliter = NGramSpliter::new(&text, 1);

    for gram in spliter{
        let tg = vec![text[i].clone()];
        assert_eq!(gram, tg);
        i += 1;
    }
}

#[test]
fn test_spliter_2() {
    let text = [1, 1, 2, 2, 3, 3, 4, 4];
    let mut i = 0;

    let spliter = NGramSpliter::new(&text, 2);

    for gram in spliter{
        let tg = vec![text[i], text[i+1]];
        assert_eq!(gram, tg);
        i += 2;
    }
}

#[test]
fn test_ngram_count_1() {
    let text = [1, 1, 2, 2, 2, 3];
    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1], 2);
    count_oracle.insert(vec![2], 3);
    count_oracle.insert(vec![3], 1);

    let count = ngram_count(&text, 1);
    assert_eq!(count, count_oracle);
}

#[test]
fn test_ngram_count_2() {
    let text = [1, 1, 2, 2, 2, 2];
    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1, 1], 1);
    count_oracle.insert(vec![2, 2], 2);

    let count = ngram_count(&text, 2);
    assert_eq!(count, count_oracle);
}

#[test]
fn test_ngram_freqency_1() {
    let text = [1, 1, 2, 2, 2, 3];
    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1], 2. / 6.);
    count_oracle.insert(vec![2], 3. / 6.);
    count_oracle.insert(vec![3], 1. / 6.);

    let count = ngram_freqency(&text, 1);
    assert_eq!(count, count_oracle);
}

#[test]
fn test_ngram_frequency_2() {
    let text = [1, 1, 2, 2, 2, 2];
    let mut count_oracle = HashMap::new();
    count_oracle.insert(vec![1, 1], 1. / 6.);
    count_oracle.insert(vec![2, 2], 2. / 6.);

    let count = ngram_freqency(&text, 2);
    assert_eq!(count, count_oracle);
}
