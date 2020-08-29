fn main() {
    let mut s = String::from("Hey there");
    println!("{}", s);

    { 
        let s1 = &mut s;
        println!("{}", s1);

        s1.push_str("yeeehaw");
        println!("{}", s1);
    }

    println!("{}", s);
}
