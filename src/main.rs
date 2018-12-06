extern crate lambda_runtime as lambda;
extern crate serde_json;

use std::error::Error;

use serde_json::Value;
use lambda::{error::HandlerError, lambda};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: Value, _: lambda::Context) -> Result<Value, HandlerError> {
    Ok(e)
}