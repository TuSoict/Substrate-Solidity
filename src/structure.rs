#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    a : Point,
    b : Point,
}

pub fn structure_example() {
    println!("Structure");

    let point_val = Point{
        x: 1.0,
        y: 2.0
    };

    let rectangle_val = Rectangle{
        a: Point {
            x: 1.1, y: 2.2 },
        b: Point {
            x: 3.0,  y: 4.5
        }
    } ;


    // if not using value , name with a "_" prefix
    let _rectangle = Rectangle {
        a: Point {
            x: 1.0,
            y: 1.0
        },
        b: Point {
            x: 2.0,
            y: 2.0
        }
    };


    println!("point_val {:?}", point_val);

    let bottom_right = Point { x: 5.2, ..point_val };   // Spread Syntax
    println!("bottom_right {:?}", bottom_right );

    println!("rectangle_val {:?}", rectangle_val );

}


