use std::fs;


fn main(){
    let res = fs::read_to_string("example.txt");

    match res{
        Ok(content) => {
            println!("File Content : {}", content);
        },
        Err(err) => {
            println!("Error : {} ", err);
        }
    }
}