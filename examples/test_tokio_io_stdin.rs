use tokio::io::stdin;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut stdin = stdin();
    let mut buf = [0; 64];
    let x = stdin.read(&mut buf).await.unwrap();
    print!("{}", std::str::from_utf8(&buf[..x]).unwrap());
}
