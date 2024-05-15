mod point;
mod elliptic_curve;
mod utils;

use crate::point::Point;
use crate::elliptic_curve::EllipticCurve;
use crate::utils::get_input;

fn main() {
    println!("Enter parameters in the following format: a b p x y k1 k2");

    let (a, b, p, x, y, k1, k2) = get_input();

    let curve = EllipticCurve::new(a, b, p);
    let point = Point::new(x, y);

    if !curve.is_on_curve(&point) {
        panic!("Point ({}, {}) is not on the curve", point.x, point.y);
    }

    let user1_public_key = curve.multiply_by_scalar(&point, k1);
    let user2_public_key = curve.multiply_by_scalar(&point, k2);

    println!("After first multiplication");
    println!("{:?}", user1_public_key);
    println!("{:?}", user2_public_key);
    
    let user1_private_key = curve.multiply_by_scalar(&user2_public_key, k1);
    let user2_private_key = curve.multiply_by_scalar(&user1_public_key, k2);

    assert_eq!(user1_private_key, user2_private_key, "Keys must be the same");
    
    println!("After second multiplication");
    println!("{:?}", user1_private_key);
    println!("{:?}", user2_private_key);
}
