use std::{path::Path, sync::Arc};

use ddt::Server;

async fn start(root_dir: impl AsRef<Path>) -> Arc<Server> {
    Server::run(root_dir.as_ref()).await.unwrap()
}

#[tokio::test]
async fn test_1() {
    let server = start("tests/server/1").await;

    let res = server.kill();
    panic!("{:?}", res);
}
