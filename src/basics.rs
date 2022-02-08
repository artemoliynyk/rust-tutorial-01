pub fn operations() {
    // mods::hello_crate::run();
    let b: i8 = -1;
    let a = {
        if b >= 1 {
            ">=1"
        } else {
            "<1"
        }
    };
    println!("a: {}", a);

    // loop
    let mut i = 1;
    let z = 'loop1: loop {
        i += 1;
        println!("loop i: {}", i);

        if i >= 13 {
            break i;
        }
    };

    println!("Loop: {}", z);

    // while
    let mut i = 10;
    while i <= 24 {
        println!("while i: {}", i);
        i += 1;
    }

    // for/each
    let t = (1, 2, 3, 4, 'a', "bc", "d");
    let t = [1, 4, 345, 456, 2, 45, 5, 3, 73, 0];
    for i in t {
        println!("Item: {}", i);
    }
}

pub fn allocations() {
    let mut s = String::from("Test");
    s.push_str("123");

    println!("s: {}", s);

    let a = 1;
    let b = a;

    println!("a: {}", b);

    let s1 = String::from("abc");
    let s2 = s1;

    println!("s: {}", s2);
}

pub fn ownership() {
    let mut s = String::from("String 123");

    take_ownership(&mut s);
    println!("post-take s: {}", s);

    let mut i = 1;
    take_int(i);

    println!("int: {}", i);

}

fn take_ownership(s: &mut String) {
    s.push('!');
    println!("String: {}", s);
}

fn take_int(i: i32) {
    println!("int: {}", i);
}