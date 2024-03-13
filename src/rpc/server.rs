use super::helloworld::greeter_server::GreeterServer;
use super::helloworld::MyGreeter;

use tonic::transport::server::Router;
use tonic::transport::Server;

pub fn build() -> Router {
    let greeter = MyGreeter::default();

    Server::builder().add_service(GreeterServer::new(greeter))
}
