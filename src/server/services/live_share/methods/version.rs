use super::super::RawMessage;
use super::super::PROTOCOL_VERSION;
use tonic::{Request, Response, Status};

pub async fn version(_request: Request<()>) -> Result<Response<RawMessage>, Status> {
    let reply = RawMessage {
        message: PROTOCOL_VERSION.to_string(),
    };
    Ok(Response::new(reply))
}
