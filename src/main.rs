use async_trait::async_trait;
use async_std;
use nvim_rs::{
  create::async_std as create, Handler,
};
use futures::io::{WriteHalf};
use async_std::os::unix::net::UnixStream;

#[derive(Clone)]
struct NeovimHandler{}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = WriteHalf<UnixStream>;
}

#[async_std::main]
async fn main() {

  let handler: NeovimHandler = NeovimHandler{};

  let (nvim, _) = create::new_unix_socket("/tmp/nvim", handler).await.unwrap();

  println!("TEST");

  nvim.command("let c = getcompletion('', 'color')").await.unwrap();
  let var = nvim.get_var("c").await;
  println!("{:?}", var);
  let var = nvim.command("colorscheme default").await;

}
