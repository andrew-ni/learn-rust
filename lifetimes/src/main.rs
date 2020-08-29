mod lib;

fn main() {
    let a = String::from("yeeeehaw");
    let b = "woohoooo";
    let c = lib::longest(&a, &b);
    println!("{}", c);
}
