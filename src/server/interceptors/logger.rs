use log::trace;
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Debug, Clone)]
pub struct RequestLogger {}

impl Interceptor for RequestLogger {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        trace!("Received request {:?}", request);
        Ok(request)
    }
}
