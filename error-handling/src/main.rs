use std::fs::File;
use std::io::ErrorKind;

mod read_username;

fn main() {
    println!("Hello, world!");
   
    read_username::read_username_from_file();

    // handling file errors
    let f = File::open("hello.txt");
    let f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
      },
    };
    
    // unwrap is a shortcut which is implemented
    // just like the match expression. 
    // It returns either Ok either Err with panic!
    let f = File::open("hello1.txt").unwrap();
    
    // expect is another shortcut that is similar to unwrap
    // it allows to specify an error message
    let f = File::open("hello1.txt").expect("Failed to open hello1.txt");

    // panic because of accessing 100th element of v
    // but v has only 3 elements
    let v = vec![1, 2, 3];
    v[99];
}
