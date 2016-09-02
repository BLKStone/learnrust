use std::num;

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        let width = (x1-x2).abs();
        let height = (y1-y2).abs();
        let area = (width * height) as f64;
        area
    }
}

fn square(p:Point, length:f32) -> Rectangle {
    let Point {x: x1, y: y1} = p;
    let p2 = Point {
        x: x1+length,
        y: y1+length,
    };
    let rec = Rectangle {
        p1: p,
        p2: p2,
    };
    rec
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let point2 = Point { x: 4.0, y: 2.0 };
    let point3 = Point { x: 6.0, y: 8.0 };
    let rec1 = Rectangle {
        p1: point2,
        p2: point3,
    };
    println!("the rectangle area is {}", rec1.rect_area());
    let point4 = Point { x: 6.0, y: 8.0 };
    let rec2 = square(point4, 4.0);
    println!("the rectangle {:?} area is {}", rec2, rec2.rect_area());

}
