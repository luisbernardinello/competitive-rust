use either::Either;
use std::f64::consts::PI;

//https://www.codewars.com/kata/5a1ebc2480171f29cf0000e5
fn main() {
    let shapes = vec![
        Either::Left((4.23, 6.43)),    // Rectangle
        Either::Right(1.23),           // Circle
        Either::Right(3.444),          // Circle
        Either::Left((1.342, 3.212)),  // Rectangle
    ];
    
    let sorted = sort_by_area(&shapes);
    
    println!("Sorted shapes by area:");
    for shape in sorted {
        match shape {
            Either::Left((w, h)) => println!("Rectangle: width={}, height={}, area={}", w, h, w * h),
            Either::Right(r) => println!("Circle: radius={}, area={}", r, PI * r * r),
        }
    }
}

fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut result = seq.to_vec();
    result.sort_by(|a, b| {
        let area_a = match a {
            Either::Left((w, h)) => w * h,
            Either::Right(r) => PI * r * r,
        };
        let area_b = match b {
            Either::Left((w, h)) => w * h,
            Either::Right(r) => PI * r * r,
        };
        area_a.partial_cmp(&area_b).unwrap()
    });
    result
}

#[cfg(test)]
mod tests {
    use super::sort_by_area;
    use either::Either;
        
    fn dotest(seq: &[Either<(f64, f64), f64>], expected: &[Either<(f64, f64), f64>]) {
        let actual = sort_by_area(seq);
        assert!(actual == expected, "With seq = {seq:?}\nExpected {expected:?} but got {actual:?}")
    }
    
    #[test]
    fn fixed_tests() {
        dotest(&[Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212))], &[Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444)]);
        dotest(&[Either::Left((2.0, 5.0)), Either::Right(6.0)], &[Either::Left((2.0, 5.0)), Either::Right(6.0) ]);
        dotest(&[], &[]);
    }
    
    #[test]
    fn sample_test() {
        let seq = &[Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212))];
        let expected = &[Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444)];
        let actual = sort_by_area(seq);
        assert_eq!(actual, expected);
    }
}