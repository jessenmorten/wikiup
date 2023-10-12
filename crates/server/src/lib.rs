use std::net::TcpListener;
use util::threads::maybe_spawn;

pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

pub fn dummy_server(config: ServerConfig) -> std::io::Result<()> {
    let addr = format!("{}:{}", config.addr, config.port);
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on http://{addr}");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        maybe_spawn(move || {
            match stream.peer_addr() {
                Ok(s) => {
                    println!("Got connection from {s}");
                }
                Err(e) => {
                    eprintln!("Failed to get peer address: {}", e);
                }
            };
        });
    }

    Ok(())
}
