
#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend:f64, divitor:f64) -> Result<f64, DivisionByZeroError> {
    if divitor==0.0 {
        Err(DivisionByZeroError)
    }else{
        Ok(dividend/divitor)
    }
}
fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
