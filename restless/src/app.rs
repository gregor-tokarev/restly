use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::requrest::Req;
use crate::route::Route;
use crate::router::RouteHandler;

pub struct App<'a> {
    routes: Vec<Route<'a>>,
}

const BASE_ADDR: &str = "127.0.0.1";

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App { routes: vec![] }
    }

    // TODO: Client error handle hook on connection
    #[tokio::main]
    pub async fn listen<F>(&mut self, port: u16, on_binded: F)
    where
        F: FnOnce(),
    {
        // TODO: Create `build_addr` function
        let addr = format!("{}:{}", BASE_ADDR.to_owned(), port);

        let listener = TcpListener::bind(addr.clone())
            .await
            .expect(format!("Can't bound at {}", addr).as_str());

        on_binded();

        loop {
            // TODO: Benchmark stream handling
            // NOTE: This can cause high latency because we awaiting result of
            // connection in main loop with `.await`
            let result = listener.accept().await;
            //                            ^^^^^^
            tokio::spawn(async {
                match result {
                    Ok((stream, addr)) => App::handle_stream(stream, addr).await,
                    Err(err) => {
                        println!("Couldn't get client: {:?}", err);
                    }
                }
            });
        }
    }

    async fn handle_stream(mut stream: TcpStream, addr: SocketAddr) {
        let (reader, mut writer) = stream.split();

        let mut buf_reader = BufReader::new(reader);
        let mut raw_req = String::new();

        buf_reader.read_to_string(&mut raw_req).await.unwrap();

        let req = Req::new(&raw_req);

        println!("Handled stream at {}", addr);
        // TODO: Parse stream
    }
}

impl RouteHandler for App<'_> {
    fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn post<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn put<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }
}