// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut y: Option<Point> = Some(Point { x: 100, y: 200 });

    if let Some(p) = y.take() {
        println!("Co-ordinates are {},{} ", p.x, p.y);
    }
    y; // Fix without deleting this line.
}
