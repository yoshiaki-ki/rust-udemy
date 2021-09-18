pub mod sub_a;
pub mod sub_b;

pub fn run() {
    // println!("Here is vars");
    // sub_a::func_a();
    // sub_b::func_b();

    // 文字列スライス型
    let s1 = "helloこんにちは挨拶"; // &str : 文字列スライス型
    let s2 = "hello";
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Static memory address s1 is {:?}", s1.as_ptr());
    println!("Static memory address s2 is {:?}", s2.as_ptr());
    println!("Len of s1 is {}", s1.len());
    println!("Len of s2 is {}", s2.len());

    // String型
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Heap memory address s1 is {:?}", s1.as_ptr());
    println!("Heap memory address s2 is {:?}", s2.as_ptr());
    println!("Len of s1 is {}", s1.len());
    println!("Len of s2 is {}", s2.len());
    println!("Capacity of s1 is {}", s1.capacity());
    println!("Capacity of s2 is {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
}
