// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
//ref borrows the variable instead of moving it to p, as such y continues to store the
//co-ordinates. Match statemens WILL consume anything provided so ref is required if you want to
//reuse variables, I.E. when printing values important values to the console.
