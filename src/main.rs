fn main(){

    // Variables In Rust

    let x: i32 = 130;
    let y: u8 = 6;
    let z: f32 = 5.6934;
    println!("x : {}, y : {}, z : {}", x, y, z);

    // // Booleans

    let is_male = true;
    let is_above_18 = true;

    if is_male{
        println!("You are Male");
    }else{
        println!("You are female, P.S. Fuck You !, There are only two genders")
    }
    
    if is_male && is_above_18{
        println!("Party bro?")
    }

    // Strings

    let greeting: String = String::from("Hey Whats Up?");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(0);
      
    // Pattern Matching
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No Character Found In Given Index"),
    }



    // Conditional & Loops

    let num: u8 = 6;

    if num % 2 == 0 {
        println!("num: {} is even", num)
    }else {
        println!("num: {} is odd", num)
    }

    // Loops

    for i in 0..10{
        print!("{} ", i);
    }

    println!("");

    // Iteration Over Array, Vector, String, Maps
    let sentence = String::from("Bhupinder Jogi");
    let first_word = get_first_word(sentence);

    println!("First Word is: {:?}", first_word);

    let a: i32 = 10;
    let b: i32 = 20;
    let ans = add_numbers(a, b);

    println!("The sum of {} and {} is {}", a, b, ans);



}


fn get_first_word(sentence: String)-> String{

    let mut ans = String::from("");

    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }

    return ans;
}


fn add_numbers(a: i32, b: i32) -> i32{
    return a + b;
}
