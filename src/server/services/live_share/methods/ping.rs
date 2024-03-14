use super::super::RawMessage;
use tonic::{Request, Response, Status};

pub async fn ping(_request: Request<RawMessage>) -> Result<Response<RawMessage>, Status> {
    let reply = RawMessage {
        message: "Pong".to_string(),
    };
    Ok(Response::new(reply))
}
