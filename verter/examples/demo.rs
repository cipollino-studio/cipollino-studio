
use verter::{File, Config};

fn main() {

    let mut file = File::open("demo.verter", Config::default()).unwrap();

    let data1 = b"Hello, World!";
    file.write_root(data1).unwrap();

    let data2 = b"What an unexpectedly lovely day!";
    let alloc = file.alloc().unwrap();
    file.write(alloc, data2).unwrap();

    drop(file); // Close the file

    let mut file = File::open("demo.verter", Config::default()).unwrap();
    assert_eq!(file.read_root().unwrap(), data1);
    assert_eq!(file.read(alloc).unwrap(), data2);

    file.delete(alloc).unwrap();

}
