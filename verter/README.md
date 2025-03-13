
# Verter

**Verter** is a file format designed for quickly modifying, deleting, or adding to parts of a file without touching the rest. This is useful for things like implementing fast autosave, since most other file formats(eg. JSON) would require you to fully rewrite the file on every save. Verter works by storing blobs of data as linked lists of fixed-sized pages called *page chains*, which can be modified and deleted individually without moving or inserting any data in the file.

This repo implements a Rust library for reading/writing to Verter files.

### Example

```rust

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

```

### Operations

Verter files support the following operations:

- `alloc() -> u64`: Allocates a page chain in the file and returns the pointer. Initially it has size 0.
- `delete(ptr: u64)`: Deletes the page chain from the file. This never actually shrinks the file - it merely marks the previously occupied parts of the file as available for new data.
- `write(ptr: u64, data: &[u8])`: Writes data to a chain. Data that was previously there gets overriden.
- `read(ptr: u64) -> Vec<u8>`: Reads data from a chain.

Verter files also have a special page chain called the "root" which you can read/write to without a pointer. This part of the file is where you can store pointers to other parts of your data.

- `write_root(data: &[u8])`: Writes data to the root
- `read_root() -> Vec<u8>`: Reads data from the root

### Namesake

The file format is named after Verter, the robot character from the 1985 soviet sci-fi epic [Guests From The Future](https://en.wikipedia.org/wiki/Guest_from_the_Future). In the series, Verter is a robot who works at the Institute of Time, archiving historical artifacts collected by time travelers. However, he wants to become a poet and is secretly in love with Polina, a time-traveling scientist. In the end, he sacrifices himself to allow Kolya and Alisa to escape from space pirates trying to steal the Melophone, a device capable of reading the thoughts of any creature in the universe.