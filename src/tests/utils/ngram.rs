use utils::ngram::*;

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
