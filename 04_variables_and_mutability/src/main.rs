#[allow(unused_variables, dead_code)]
fn main() {
    // x is mutable
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Constants - Bound to a name and are not allowed to change.
    //
    // No allowed to use `mut` with constants -- They're always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let y = 7;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Another shadowing
    //
    // Shadow lets me creating a new variable when use the `let` keyword again,
    // change the the type of the value but reuse the same name.
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The length of spaces is: {spaces}");
}
