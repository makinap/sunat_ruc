use std::fs::File;
use std::io::{BufReader, BufRead};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::header::CONTENT_TYPE;
use hyper::service::{make_service_fn, service_fn};
use std::time::Instant;
use url::Url;
use std::collections::HashMap;

#[tokio::main]
async fn main() {

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn get_data(_ruc: &str) -> String {
    let now = Instant::now();

    let mut line: String = String::new();
    let file = File::open("/tmp/padron_reducido_ruc.txt").unwrap();
    let reader = BufReader::new (DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(file));
    for cline in reader.lines() {
        line = String::clone(&mut cline.unwrap());
        if &line[0..11] == _ruc {
            println!("{}", line);
            break;
        }
    }
    println!("{}ms", now.elapsed().as_millis());
    return line;
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {

    let params: HashMap<String, String> = _req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    let ruc = params.get("ruc").unwrap();

    println!("{}", ruc);
    //10728723799
    //10060872886
    //20545648301
    let _ruc = get_data(ruc);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(_ruc)).unwrap();
    Ok(response)
}