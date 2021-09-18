// Generics を作ることで任意のデータ型を取れる
struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let num_list = vec![34, 20, 50, 99, 75];
    // let mut largest = num_list[0];
    // for num in num_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largest_i32(num_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
    println!("{}", largest(num_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.5 };
    let p3 = PointAnother { x: 1.0, y: 2 };
    let p4 = PointAnother {
        x: "Rust",
        y: "Python",
    };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
