fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let l = largest2(&number_list);
    println!("The largest number is {}", l);

    // let p = Point { x: 4, y: 5 };
    // println!("p.x = {}", p.x());
}

// fn largest(list: &[i32]) -> i32 {
//     let mut l = list[0];
//     for number in list {
//         if *number > l {
//             l = *number;
//         }
//     }
//     l
// }

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut l = list[0];
    for &item in list {
        if item > l {
            l = item;
        }
    }
    l
}

fn largest2<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut l = &list[0];
    for item in list {
        if *item > *l {
            l = item;
        }
    }
    &l
}

struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn mixup2<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}