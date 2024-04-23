use std::io;

fn main() {
    loop{
        println!("Enter a string: ");
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Error reading line");
        println!("You typed {}", input_string);
        break;
    }
}
