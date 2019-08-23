use actix_web::http::Method;
use actix_web::server;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use rand;

pub const LOCAL_IP: &str = "0.0.0.0";
pub const ROOT_GET_RESPONSE: &str = "Hello world!";

fn main() {
    let arg = std::env::args().nth(1);
    let port = arg
        .and_then(|p| p.parse::<u16>().ok().map(|p| p.to_string()))
        .unwrap_or_else(|| String::from("8080"));
    println!("{}", port);

    let binding = format!["{}:{}", LOCAL_IP, port];

    println!("Starting server on {} ...", binding);
    if let Ok(s) = server::new(|| {
        vec![
            App::new()
                .prefix("/random")
                .resource("/", |r| r.method(Method::GET).f(positive))
                .finish(),
            App::new()
                .prefix("/negative")
                .resource("/", |r| r.method(Method::GET).f(negative))
                .finish(),
        ]
    })
    .bind(binding)
    {
        println!("binding successful");
        s.run();
    } else {
        println!("could not resolve binding");
    }
    println!("Goodbye!");
}

fn positive(req: &HttpRequest) -> impl Responder {
    println!("Request received: {:?}", req);
    let r = rand::random::<u8>();
    let r = format!("{}", r);

    println!("Sending payload: {}", r);
    HttpResponse::Ok().body(r)
}

fn negative(req: &HttpRequest) -> impl Responder {
    println!("Request received: {:?}", req);
    let r = rand::random::<u8>();
    let r = format!("-{}", r);

    println!("Sending payload: {}", r);
    HttpResponse::Ok().body(r)
}
