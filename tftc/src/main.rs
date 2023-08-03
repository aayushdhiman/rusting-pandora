use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let file_path = &args[1];
    let operation = &args[2];

    println!("File {}", file_path);
    if operation == "e" {
        println!("Operation: encrypt");
        if args.len() == 6 {
            let _method = &args[3];
            let _mode = &args[4];
            let _password = &args[5];
        } else {
            panic!("Not enough arguments provided to encrypt the file.")
        }
        
    } else if operation == "d" {
        println!("Operation: decrypt");
    } else {
        println!("Operation {}", operation);
    }
}