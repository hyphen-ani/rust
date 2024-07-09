struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    // let mut str: String = String::from("Hello");
    // update_str(&mut str);
    // println!("{}", str);


    let user1 = User{
        active: true,
        username: String::from("anirudh"),
        email: String::from("anirudh.hyphen@icloud.com"),
        sign_in_count: 54,
    };

    println!("Active {}, Username: {}, Email: {} ", user1.active, user1.username, user1.email);
}

// fn update_str(str: &mut String){
//     str.push_str(", world");
// }