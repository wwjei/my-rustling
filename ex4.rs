// ex4.rs
// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    let result: Result<i32, std::num::ParseIntError>  = "1".parse();
    match result {
        Ok(x) => Ok(x * 4),
        Err(e) => Err(e)
    }
}

fn something2() -> Result<i32, std::num::ParseIntError> {
    let result :i32 = "11".parse()?;
    Ok(result * 4)
}

fn main() {
    match something() {
        Ok(a) => println!("You win! {:?}", a),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }

    match something2() {
        Ok(a) => println!("You win! {:?}", a),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
