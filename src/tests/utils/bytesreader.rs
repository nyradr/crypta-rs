use std::io::Read;
use utils::bytesreader::BytesReader;

#[test]
fn test_bytesreader1(){
    let bytes = [1, 2, 3, 4, 5];
    let mut br = BytesReader::new(&bytes);
    let mut buff = [0; 5];

    assert!( match br.read(&mut buff){
        Ok(s) => s == 5,
        _ => false
    });
    assert_eq!(buff, bytes);
}

#[test]
fn test_bytesreader2(){
    let bytes = [];
    let mut br = BytesReader::new(&bytes);
    let mut buff = [0; 5];

    assert!( match br.read(&mut buff){
        Ok(s) => s == 0,
        _ => false
    });
    assert_eq!(buff, [0; 5]);
}

#[test]
fn test_bytesreader3(){
    let bytes = [1, 2, 3, 4, 5];
    let mut br = BytesReader::new(&bytes);
    let mut buff = [0; 2];

    match br.read(&mut buff){
        Ok(s) => {
            assert_eq!(s, 2);
            assert_eq!(buff, [1, 2]);
        },
        _ => assert!(false)
    };

    match br.read(&mut buff){
        Ok(s) => {
            assert_eq!(s, 2);
            assert_eq!(buff, [3, 4]);
        },
        _ => assert!(false)
    };

    match br.read(&mut buff){
        Ok(s) => {
            assert_eq!(s, 1);
            assert_eq!(buff, [5, 4]);
        },
        _ => assert!(false)
    };
}