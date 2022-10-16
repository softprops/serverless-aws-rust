use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Request {
    question: u32
}

#[derive(Serialize, Debug, PartialEq)]
struct Response {
    answer: u32
}

async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    Ok(Response {
        answer: event.payload.question
    })
}

#[cfg(test)]
mod tests {
    use lambda_runtime::Context;

    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let event = LambdaEvent {
            payload: Request {
                question: 42
            },
            context: Context::default()
        };
        let response = Response {
            answer: event.payload.question
        };
        assert_eq!(
            handler(event)
                .await
                .expect("expected Ok(_) value"),
            response
        )
    }
}
