fn main() {
    let x = five();
    let x = plus_one(x);

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
