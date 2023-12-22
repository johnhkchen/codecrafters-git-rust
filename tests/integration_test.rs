use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

#[test]
fn test_assert_fs_basic_syntax() {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();
    //(b"Hello, world!");
    input_file.write_str("Hello, World!").unwrap();
    input_file.assert("Hello, World!");
    temp.child("bar.txt").assert(predicate::path::missing());
    temp.close().unwrap();
}

#[test]
fn test_tempfile_read_write() {
    // Write
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);
}
