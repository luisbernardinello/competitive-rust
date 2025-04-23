use std::io;
//https://judge.beecrowd.com/pt/problems/view/1041
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let coords: Vec<f32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let (x, y) = (coords[0], coords[1]);
    
    let result = identify_quadrant(x, y);
    
    println!("{}", result);
}

fn identify_quadrant(x: f32, y: f32) -> &'static str {
    match (x == 0.0, y == 0.0) {
        (true, true) => "Origem",
        (false, true) => "Eixo X",
        (true, false) => "Eixo Y",
        (false, false) => match (x > 0.0, y > 0.0) {
            (true, true) => "Q1",
            (true, false) => "Q4",
            (false, true) => "Q2",
            (false, false) => "Q3",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrants() {
        // Test case 1: Quadrant 4 (positive x, negative y)
        assert_eq!(identify_quadrant(4.5, -2.2), "Q4");
        
        // Test case 2: Quadrant 1 (positive x, positive y)
        assert_eq!(identify_quadrant(0.1, 0.1), "Q1");
        
        // Test case 3: Origin
        assert_eq!(identify_quadrant(0.0, 0.0), "Origem");
        
        // Additional cases for complete coverage
        assert_eq!(identify_quadrant(-1.0, 2.0), "Q2");
        assert_eq!(identify_quadrant(-3.0, -4.0), "Q3");
        assert_eq!(identify_quadrant(0.0, 5.0), "Eixo Y");
        assert_eq!(identify_quadrant(-7.0, 0.0), "Eixo X");
    }
}