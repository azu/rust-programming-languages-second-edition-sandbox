// rust buildはlib.rsから見る
pub mod network;
pub mod client;
pub mod server;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
