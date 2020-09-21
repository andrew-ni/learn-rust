fn main() {
    let c = CustomSmartPointer {
        data: String::from("hey"),
    };
    let d = CustomSmartPointer {
        data: String::from("bye"),
    };
    println!("CustomSmartPointers Created");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
