use lesson4::traffic_lights::*;
use lesson4::arr_sum::*;
use lesson4::shape_area::*;
use lesson4::shape_area::Shape::{Circle, Triangle, Rectangle};

fn main() {
    let green = TrafficLights::GREEN(10);
    println!("{}", green.as_str());
    let arr: [u32; 3] = [4294967295, 2, 3];
    match sum(&arr) {
        Some(r) => println!("sum arr result: {}", r),
        _ => println!("error")
    }
    println!("Circle r = 10.0 area is : {}", Circle { r: 10.0 }.get_area());
    println!("Triangle a = 6 b = 6 c = 6  area is : {}", Triangle { a: 6.0, b: 6.0, c: 6.0 }.get_area());
    println!("Rectangle width = 10 height = 20  area is : {}", Rectangle {width: 10.0, height: 20.0}.get_area());
}
