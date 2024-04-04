
use my_predictor_lib;

fn main() {
    if let Err(e) = my_predictor_lib::test222() {
        println!("Writing error: {}", e.to_string());   
    }
}