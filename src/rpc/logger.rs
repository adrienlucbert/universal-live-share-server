use jsonrpsee::server::middleware::rpc::RpcServiceT;
use jsonrpsee::types::Request;

#[derive(Clone)]
pub struct Logger<S>(pub S);

impl<'a, S> RpcServiceT<'a> for Logger<S>
where
    S: RpcServiceT<'a> + Send + Sync,
{
    type Future = S::Future;

    fn call(&self, req: Request<'a>) -> Self::Future {
        println!("{}", req.method);
        self.0.call(req)
    }
}
