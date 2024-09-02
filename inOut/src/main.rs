use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main(){
    println!("Hello, world!");
    
    let mut input = String::new();

    print!("Enter some text, any test: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("{}", input.trim());

    println!("The End. Goodby");


}
