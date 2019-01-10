// rust buildはlib.rsから見る
pub mod network;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect;
    }
}
