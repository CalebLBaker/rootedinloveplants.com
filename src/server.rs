use lettre::smtp::authentication::IntoCredentials;
use lettre::Transport;

use serde::Deserialize;

mod credentials;

const SERVER_ROOT : &str = "/content";
// const SERVER_ROOT : &str = "/home/caleb/src/rootedinloveplants";
const DESTINATION : &str = "rootedinloveplants@gmail.com";

#[derive(Deserialize)]
struct EmailRequest {
    reply_to: String,
    subject: String,
    body: String,
}

struct EmailError(hyper::StatusCode);

impl From<std::str::Utf8Error> for EmailError {
    fn from(_: std::str::Utf8Error) -> Self {
        EmailError(hyper::StatusCode::BAD_REQUEST)
    }
}

impl From<serde_json::Error> for EmailError {
    fn from(_: serde_json::Error) -> Self {
        EmailError(hyper::StatusCode::BAD_REQUEST)
    }
}

impl From<lettre::smtp::error::Error> for EmailError {
    fn from(_: lettre::smtp::error::Error) -> Self {
        EmailError(hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<hyper::Error> for EmailError {
    fn from(_: hyper::Error) -> Self {
        EmailError(hyper::StatusCode::BAD_REQUEST)
    }
}

impl From<lettre_email::error::Error> for EmailError {
    fn from(_: lettre_email::error::Error) -> Self {
        EmailError(hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// Parse email contents from JSON in a request body and send the email
async fn send_email(req: hyper::Request<hyper::Body>) -> Result<(), EmailError> {
    let body = hyper::body::to_bytes(req).await?.to_vec();
    let json: EmailRequest = serde_json::from_str(std::str::from_utf8(body.as_slice())?)?;
    let email_builder = lettre_email::EmailBuilder::new().to(DESTINATION).from(credentials::EMAIL);
    let email = email_builder.reply_to(json.reply_to).subject(json.subject).text(json.body).build()?.into();
    let credentials = (credentials::EMAIL, credentials::PASSWORD).into_credentials();
    lettre::SmtpClient::new_simple("smtp.gmail.com")?.credentials(credentials).transport().send(email)?;
    Ok(())
}

fn err_to_body<T: ToString>(e : T) -> hyper::Body {
    hyper::Body::from(e.to_string().into_bytes())
}

#[tokio::main]
async fn main() {
    // Get the address to serve on
    let port = 8080;
    let addr = ([0, 0, 0, 0], port).into();

    // Run the service
    let _ = hyper::Server::bind(&addr).serve(hyper::service::make_service_fn(|_| async {
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(|req| async move {

            let method = req.method();
            let builder = hyper::Response::builder();

            // Respond to GET requests more verbosely since the response will be directly rendered in a browser.
            // On success, just throw the contents of the requested file into the body of an OK response
            if method == hyper::Method::GET {
                let uri_path;
                match urlencoding::decode(req.uri().path()) {
                    Ok(p) => uri_path = p,
                    Err(e) => return builder.status(hyper::StatusCode::BAD_REQUEST).body(err_to_body(e)),
                };

                let server_root_path = std::path::Path::new(SERVER_ROOT);

                // Redirect an empty path to index.html, otherwise get the canonical path
                let path_result = if uri_path.is_empty() || uri_path == "/" {
                    Ok(server_root_path.join("index.html"))
                }
                else {
                    server_root_path.join(&uri_path[1..]).canonicalize()
                };
                match path_result {
                    Ok(path) => {
                        // Make sure the request doesn't try to escape the server root directory.
                        // In particular, serving the server executable would be a security
                        // vulnerability
                        if path.starts_with(SERVER_ROOT) {
                            match std::fs::read(path.clone()) {
                                Ok(contents) => {
                                    let mut response = builder.status(hyper::StatusCode::OK);
                                    // Set mime type
                                    if let Some(extension) = path.extension() {
                                        response = match extension.to_str() {
                                            Some("html") => response.header("Content-Type", "text/html"),
                                            Some("css") => response.header("Content-Type", "text/css"),
                                            Some("js") => response.header("Content-Type", "text/javascript"),
                                            Some("png") => response.header("Content-Type", "image/png"),
                                            Some("jpg") => response.header("Content-Type", "image/jpeg"),
                                            Some("webp") => response.header("Content-Type", "image/webp"),
                                            _ => response,
                                        }
                                    }
                                    response.body(hyper::Body::from(contents))
                                }
                                Err(e) => builder.status(hyper::StatusCode::NOT_FOUND).body(err_to_body(e))
                            }
                        }
                        else {
                            builder.status(hyper::StatusCode::FORBIDDEN).body(hyper::Body::empty())
                        }
                    }

                    Err(e) => builder.status(hyper::StatusCode::NOT_FOUND).body(err_to_body(e))
                }
            }

            // Handle post requests by sending emails
            else if method == hyper::Method::POST {
                match send_email(req).await {
                    Ok(_) => builder.status(hyper::StatusCode::OK).body(hyper::Body::empty()),
                    Err(e) => builder.status(e.0).body(hyper::Body::empty()),
                }
            }
            // We don't handle any other kind of request besides get and post
            else {
                builder.status(hyper::StatusCode::METHOD_NOT_ALLOWED).body(hyper::Body::empty())
            } 
        }))
    })).await;

}

