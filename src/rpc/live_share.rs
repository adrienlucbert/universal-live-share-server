tonic::include_proto!("live_share");

pub const PROTOCOL_VERSION: &str = "0.0.1";

mod live_share_impl;

mod methods;
pub use live_share_impl::LiveShareImpl;
