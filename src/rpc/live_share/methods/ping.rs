use super::super::RawMessage;
use tonic::{Request, Response, Status};

pub async fn ping(request: Request<RawMessage>) -> Result<Response<RawMessage>, Status> {
    println!("Got a request: {:?}", request);

    let reply = RawMessage {
        message: "Pong".to_string(),
    };

    Ok(Response::new(reply))
}
