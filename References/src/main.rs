fn main() {
    let mut str: String = String::from("Hello");
    update_str(&mut str);
    println!("{}", str);
}

fn update_str(str: &mut String){
    str.push_str(", world");
}