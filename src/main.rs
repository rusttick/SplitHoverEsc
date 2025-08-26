fn greater_than_5(x: i32) -> i32 {
    if x > 5 {
        1
    } else {
        0
    }
}

fn main() {
    const THING_NEVER_CHANGES: u32 = 56;

    println!("the constant is {THING_NEVER_CHANGES}");

    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    let t = (500, 'c', 3.9);
    let (num, letter, floater) = t;

    let f = t.2;

    println!("num = {num}, letter = {letter}, floater = {floater}, f is also {f}");

    let ary = [1, 2, 3, 3, 4];

    println!("ary 1 = {}", ary[1]);

    let y = greater_than_5(10);
    println!("y is {}", y);
}
