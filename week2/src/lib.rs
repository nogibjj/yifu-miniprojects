// Create a dice function where six-sided dice are thrown, and each side has an equal probability of appearing
pub fn squarenum(number: &str) -> String {
    let num: i32 = number.parse().unwrap();
    let result = num * num;
    result.to_string()
}