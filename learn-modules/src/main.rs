mod outer;
mod other;
use outer::inner as inner;

fn main() {
    println!("Hello, world!");
    outer::outer1::outer1function(1);
    outer::inner::inner1::inner1function(1);
    other::inside::inside1::inside1function();
    other::other1::other1function();
    inner::inner1::inner1function(1);
}
