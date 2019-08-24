use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
    let sys = actix::System::new("Bleep server");

    let server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/random").route("/", web::get().to(positive)))
            .service(web::scope("/negative").route("/", web::get().to(negative)))
            .service(web::scope("/prime").route("/{number}", web::get().to(prime)))
            .route("/", web::to(|| HttpResponse::Ok().body("Bleep!")))
    })
    .bind(binding);
    if let Ok(s) = server {
        println!("binding successful");
        s.start();
        let _ = sys.run();
    } else {
        println!("could not resolve binding");
    }

    println!("Goodbye!");
}

fn is_prime(candidate: u32) -> bool {
    match candidate {
        x if x <= 3 => x > 1,
        x if x % 2 == 0 || x % 3 == 0 => false,
        n => {
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}

fn prime(params: HttpRequest) -> impl Responder {
    println!("Prime check request received: {:?}", params );
    let candidate = params
        .match_info()
        .get("number")
        .and_then(|n| n.parse::<u32>().ok())
        .expect("Could not parse requested number");
    if is_prime(candidate) {
        format!["{} is prime!", candidate]
    } else {
        format!["Nope, {} is not prime.", candidate]
    }
}

fn positive() -> impl Responder {
    println!("Request for positive random number");
    let r = rand::random::<u8>();
    let r = format!("{}", r);

    println!("Sending payload: {}", r);
    HttpResponse::Ok().body(r)
}

fn negative() -> impl Responder {
    println!("Request for negative random number");
    let r = rand::random::<u8>();
    let r = format!("-{}", r);

    println!("Sending payload: {}", r);
    HttpResponse::Ok().body(r)
}
