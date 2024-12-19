#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // This code matches an Option<Point> value
    // Using ref p borrows the Point value inside Some instead of moving it
    // This allows us to still use optional_point after the match
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Print the original optional_point
}
