use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

fn operate_file() {
    let file = File::create("./test.txt");
    
    let open_file = File::open("./test.txt").expect("File not found.");

    let open_file2 = OpenOptions::new().write(true).create(true).open("./test.txt");

    fs::copy("./test.txt", "./test_copy.txt").expect("copy failed.");


    let byte_arr = fs::read("./test.txt").expect("read failed.");
    println!("value={}", String::from_utf8(byte_arr).unwrap());

    let str1 = fs::read_to_string("./test.txt").expect("read failed.");
    println!("value={}", str1);

    fs::write("./test.txt", "Hello world for rust").expect("write failed.");

    let file_meta = fs::metadata("./test.txt");

}

fn operator_path() {

    let dir_entries = fs::read_dir(".");
    for entry in dir_entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let meta = entry.metadata().unwrap();
        let name = entry.file_name();
    }
}

fn main() {
    operate_file();
}
