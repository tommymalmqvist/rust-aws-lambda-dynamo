use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error};
use serde::{Serialize, Deserialize};
use simple_error::bail;
use simple_logger;

#[derive(Deserialize, Clone)]
struct Event {
    #[serde(rename = "userId")]
    user_id: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(event_handler);

    Ok(())
}

fn event_handler(e: Event, c: Context) -> Result<CustomOutput, HandlerError> {
    if e.user_id == "" {
        error!("Empty user id in request {}", c.aws_request_id);
        bail!("Empty user id");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.user_id),
    })
}