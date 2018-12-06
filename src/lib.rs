extern crate lambda_runtime as lambda;
extern crate serde_json;

use serde_json::Value;
use lambda::{error::HandlerError, lambda};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: Value, c: lambda::Context) -> Result<Value, HandlerError> {
    Ok(e)
}