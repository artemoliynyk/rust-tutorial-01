pub fn main() {
    let mut s = String::from("String 123");

    s = take_ownership(s);
    println!("post-take s: {}", s);
    
    modify_string(&mut s);
    println!("post-modify s: {}", s);

    let s1 = &s;
    println!("s1: {}", s1);

    let s2 = &mut s;
    println!("s2: {}", s2);

    let i = 1;
    take_int(i);

    println!("int: {}", i);

    let d = dangle();
    println!("d: {}", d);
}

fn take_ownership(s: String) -> String {
    println!("String: {}", s);

    s
}
fn modify_string(s: &mut String) {
    s.push('!');
}

fn take_int(i: i32) {
    println!("int: {}", i);
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
