use std::fs::File;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    let file_path = "hello.txt";
    get_file_handle(file_path)?;
    Ok(())
}

fn get_file_handle(file_path: &str) -> io::Result<File> {
    match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let fc = File::create(file_path)?; // 这里用 ?
            println!("File {file_path} not found, created a new one");
            Ok(fc)
        }
        Err(e) => Err(e),
    }
}

// fn panic_and_print(err: std::io::Error) -> ! {
//     panic!("Problem occurred: {err:#?}")
// }
//
// fn get_file_handle(file_path: &str) -> File {
//     let file_result = File::open(file_path);
//     match file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create_new(file_path) {
//                 Ok(fc) => {
//                     println!("File {file_path} not found, created a new one");
//                     fc
//                 }
//                 Err(err) => panic_and_print(err),
//             },
//             _ => panic_and_print(error),
//         },
//     }
// }
