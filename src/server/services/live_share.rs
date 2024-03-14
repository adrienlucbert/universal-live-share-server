mod live_share_impl;
mod methods;

pub use live_share_impl::LiveShareImpl;

tonic::include_proto!("live_share");

pub const PROTOCOL_VERSION: &str = "0.0.2";
