enum List {
    Cons(i32, Box<List>),
    Nil,
}

// use crate::List::{Cons, Nil};
// use self::List::{Cons, Nil};
use List::{Cons, Nil};


fn main() {
    let list = Cons(1, Box::new(Cons(1, Box::new(Cons(2, Box::new(Nil))))));
}
