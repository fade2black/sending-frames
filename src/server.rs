use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![];
            loop {
                gen_rand_bytes(&mut buf);

                println!("Sending {:?}", buf);

                if let Err(e) = socket.write_all(&buf).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }

                // wait
                let secs = rand::random::<u64>() % 5 + 1;
                sleep(Duration::from_secs(secs)).await;
            }
        });
    }
}

fn gen_rand_bytes(buf: &mut Vec<u8>) {
    buf.clear();
    let n = rand::random::<u8>() % 5 + 1;

    buf.push(n);
    for _ in 0..n {
        buf.push(rand::random::<u8>() % 7 + 1)
    }
}
