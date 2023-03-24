// Create a function to convert a decimal to hexadecimal and binary
pub fn decimaltohexadecimal(number: i32) -> String {
    let mut number = number;
    let mut result = String::new();
    let mut _remainder = 0;
    while number != 0 {
        _remainder = number % 16;
        number /= 16;
        match _remainder {
            10 => result.push('A'),
            11 => result.push('B'),
            12 => result.push('C'),
            13 => result.push('D'),
            14 => result.push('E'),
            15 => result.push('F'),
            _ => result.push_str(&_remainder.to_string()),
        }
    }
    result.chars().rev().collect()
}
