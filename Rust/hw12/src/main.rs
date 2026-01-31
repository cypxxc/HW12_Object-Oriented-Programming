use std::f64::consts::PI;

trait Shaper {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn info(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shaper for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn info(&self) {
        println!("Rectangle:");
        println!("width: {} height: {}", self.width, self.height);  
        println!("Area: {:.2}", self.area());
        println!("Perimeter: {:.2}", self.perimeter());
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shaper for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn info(&self) {
        println!("Circle:");
        println!("radius: {}", self.radius);
        println!("Area: {:.2}", self.area());
        println!("Perimeter: {:.2}", self.perimeter());
    }
}   

fn main() {
    let r = Rectangle::new(10.0, 20.0);
    let c = Circle::new(10.0);

    r.info();
    println!("");
    c.info();
}
