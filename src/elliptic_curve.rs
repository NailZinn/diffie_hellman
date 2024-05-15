use crate::utils::get_modular_inverse;
use crate::point::Point;

pub struct EllipticCurve {
    a: i32,
    b: i32,
    p: i32
}

impl EllipticCurve {
    pub fn new(a: i32, b: i32, p: i32) -> Self {
        if !EllipticCurve::check_elliptic_curve(a, b, p) {
            panic!("Elliptic curve with coefficients a = {}, b = {} and modulo p = {} is invalid", a, b, p);
        }
        Self { a, b, p }
    }

    pub fn is_on_curve(&self, point: &Point) -> bool {
        let (x, y) = (point.x, point.y);
        (y * y - x * x * x - self.a * x - self.b).rem_euclid(self.p) == 0
    }

    pub fn check_elliptic_curve(a: i32, b: i32, p: i32) -> bool {
        (4 * a * a * a + 27 * b * b).rem_euclid(p) != 0
    }

    pub fn multiply_by_scalar(&self, point: &Point, mut scalar: i32) -> Point {
        let mut result = Point::NONE;
        let mut acc = Point::new(point.x, point.y);

        while scalar != 0 {
            if scalar & 1 == 1 {
                result = self.add(&result, &acc);
            }
            acc = self.add(&acc, &acc); 
            scalar >>= 1; 
        }

        result
    }

    pub fn add(&self, point1: &Point, point2: &Point) -> Point {
        if point1 == &Point::NONE {
            return Point::new(point2.x, point2.y);
        }
        
        if point2 == &Point::NONE {
            return Point::new(point1.x, point1.y);
        }

        if point1.x == point2.x && point1.y != point2.y {
            return Point::NONE;
        }

        let (x1, y1) = (point1.x, point1.y);
        let (x2, y2) = (point2.x, point2.y);
        
        let lambda = if x1 == x2 {
            ((3 * x1 * x1 + self.a) * get_modular_inverse(2 * y1, self.p)).rem_euclid(self.p)
        } else {
            ((y2 - y1) * get_modular_inverse(x2 - x1, self.p)).rem_euclid(self.p)
        };

        let x = (lambda * lambda - x1 - x2).rem_euclid(self.p);
        let y = (y1 + lambda * (x - x1)).rem_euclid(self.p);
        let result = Point::new(x, y);

        assert!(self.is_on_curve(&result), "Point ({}, {}) is not on the curve", x, y);

        result
    }
}