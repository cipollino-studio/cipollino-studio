
use crate::{Config, Error, File};

#[test]
fn hello_world() {
    let mut file = File::open("hello.verter", Config::default()).unwrap();
    let data = b"Hello, World!".to_owned(); 
    file.write_root(&data).unwrap();

    drop(file);

    let mut file = File::open("hello.verter", Config::default()).unwrap();
    assert_eq!(&data, file.read_root().unwrap().as_slice());
    std::fs::remove_file("hello.verter").unwrap();
}

#[test]
fn deletion() {
    let mut file = File::open("deletion.verter", Config::default()).unwrap();
    let page = file.alloc().unwrap();
    file.write(page, b"Hey there").unwrap();
    file.delete(page).unwrap();
    let new_page = file.alloc().unwrap();
    assert_eq!(page, new_page); // Deleted page should be re-used
    std::fs::remove_file("deletion.verter").unwrap();
}

#[test]
fn truncation() {
    let mut file = File::open("truncation.verter", Config::default()).unwrap();
    file.write_root(&vec![0xAE; 2000]).unwrap();
    file.write_root(&vec![0xBA; 200]).unwrap();
    drop(file);

    let file_size = std::fs::metadata("truncation.verter").unwrap().len();

    let mut file = File::open("truncation.verter", Config::default()).unwrap();
    file.alloc().unwrap();
    drop(file);

    let new_file_size = std::fs::metadata("truncation.verter").unwrap().len();

    assert_eq!(file_size, new_file_size);

    std::fs::remove_file("truncation.verter").unwrap();
} 

#[test]
fn magic_bytes() {
    let file = File::open("magic_bytes.verter", Config {
        magic_bytes: b"Magic1",
        ..Config::default()
    }).unwrap();
    drop(file);

    match File::open("magic_bytes.verter", Config {
        magic_bytes: b"Magic2",
        ..Config::default()
    }) {
        Err(Error::InvalidFile) => {},
        Ok(_) | Err(_) => panic!("should error with invalid file")
    }

    std::fs::remove_file("magic_bytes.verter").unwrap();
}

#[test]
fn invalid_pointer() {
    let mut file = File::open("invalid_pointer.verter", Config::default()).unwrap();

    match file.read(3) {
        Err(Error::InvalidPointer) => {}
        Ok(_) | Err(_) => panic!("should error with invalid pointer")
    }

    match file.read(file.header_size() + 10000 * file.total_page_size()) {
        Err(Error::InvalidPointer) => {}
        Ok(_) | Err(_) => panic!("should error with invalid pointer")
    }

    let alloc = file.alloc().unwrap();
    file.delete(alloc).unwrap();
    match file.read(alloc) {
        Err(Error::DeletedPointer) => {},
        Ok(_) | Err(_) => panic!("should error with deleted pointer")
    }

    std::fs::remove_file("invalid_pointer.verter").unwrap();
}

#[test]
fn extension() {
    let mut file = File::open("extension.verter", Config::default()).unwrap();
    let alloc = file.alloc().unwrap();
    drop(file);

    for i in 0..100 {
        let size = i * 45;
        let next_size = (i + 1) * 45;

        let mut file = File::open("extension.verter", Config::default()).unwrap();
        let old_data = file.read(alloc).unwrap();
        assert_eq!(old_data, vec![0xFA; size]);
        file.write(alloc, &vec![0xFA; next_size]).unwrap();
    }
    
    std::fs::remove_file("extension.verter").unwrap();
}
