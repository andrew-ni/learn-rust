pub fn outer1function(num: i32) -> i32 {
    num + 2
}

use super::inner::inner1;
use crate::other::other1;
use crate::other::inside;

fn f() {
    inner1::inner1function(1);
    other1::other1function();
    inside::inside1::inside1function();
}