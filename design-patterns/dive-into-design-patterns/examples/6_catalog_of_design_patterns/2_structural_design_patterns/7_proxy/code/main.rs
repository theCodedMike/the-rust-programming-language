use crate::server::{NginxServer, Server};

mod server;

/// cargo r --example proxy
fn main() {
    let app_status = &"/app/status".to_string();
    let create_user = &"/create/user".to_string();

    let mut nginx = NginxServer::new();

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(app_status, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx.handle_request(create_user, "POST");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

    let (code, body) = nginx.handle_request(create_user, "GET");
    println!("Url: {}\nHttpCode: {}\nBody: {}\n", create_user, code, body);

    // Url: /app/status
    // HttpCode: 200
    // Body: Ok
    //
    // Url: /app/status
    // HttpCode: 200
    // Body: Ok
    //
    // Url: /app/status
    // HttpCode: 403
    // Body: Not Allowed
    //
    // Url: /create/user
    // HttpCode: 201
    // Body: User Created
    //
    // Url: /create/user
    // HttpCode: 404
    // Body: Not Ok
}
