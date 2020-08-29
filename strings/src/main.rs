fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    // println!("{}", s1);

    let hello = "andrewni";
    for b in hello.bytes() {
        println!("{}", b);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
