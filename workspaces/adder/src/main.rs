use add_one;
use add_two;
use add_three;

fn main() {
    println!("Hello, world!");
    println!("{}", add_one::add_one(1));
    println!("{}", add_two::add_two(2));
    println!("{}", add_three::add_three(3));

}
