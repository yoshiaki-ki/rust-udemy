enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // stack overflow
    // let a1: [u8; 7000000] = [1; 7000000]; // 7MBのデータ
    // let a2: [u8; 9000000] = [1; 9000000]; // 9MBのデータ stackは最大8MBまでのデータを格納できる

    // Vector型
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Stack address of v1 is {:p}", &v1);
    println!("Stack address of v2 is {:p}", &v2);
    println!("Stack address of v3 is {:p}", &v3);
    println!("Heap memory address of v1 is {:?}", v1.as_ptr());
    println!("Heap memory address of v2 is {:?}", v2.as_ptr());
    println!("Heap memory address of v3 is {:?}", v3.as_ptr());

    // Box pointer
    // 作る前
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of t1 is {:p}", &t1);
    println!("Heap memory address of t1.1 is {:?}", t1.1.as_ptr());

    // Box pointerを作る
    let mut b1 = Box::new(t1);
    (*b1).1 += "World";
    println!("{} {}", b1.0, b1.1);
}
