use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    runtime, select,
    sync::mpsc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:9999").await?;

    let (mut read, mut write) = stream.into_split();

    // NOTE: use tokio::io::stdin
    let mut stdin = tokio::io::stdin();

    // let (in_tx, mut in_rx) = mpsc::channel(10);
    // std_io_read_stdin_lines(in_tx.clone(), runtime::Handle::current());

    let mut buf = [0; 64];
    let mut buf_stdin = [0; 64];

    loop {
        select! {
            Ok(res) = read.read(&mut buf) => {
                    println!("S: {:?}", std::str::from_utf8(&buf[..res]).unwrap());
            }
            // Some(buffer) = in_rx.recv() => {
            //     write.write_all(buffer.as_bytes()).await.unwrap();
            // }
            Ok(res) = stdin.read(&mut buf_stdin) => {
                write.write_all(&buf_stdin[..(res-1)]).await.unwrap();
            }
        }
    }
}

// NOTE: using std::io to read from stdin
fn std_io_read_stdin_lines(sender: mpsc::Sender<String>, runtime: runtime::Handle) {
    // Tutor: https://stackoverflow.com/questions/69665398/handling-user-input
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut line_buf = String::new();
        while let Ok(_) = stdin.read_line(&mut line_buf) {
            let line = line_buf.trim_end().to_string();
            line_buf.clear();
            let sender2 = sender.clone();

            runtime.spawn(async move {
                let result = sender2.send(line).await;
                if let Err(error) = result {
                    println!("start_reading_stdin_lines send error: {:?}", error);
                }
            });
        }
    });
}
