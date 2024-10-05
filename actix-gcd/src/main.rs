use actix_web::{get,post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
            .service(post_gcd)
            })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0{
        return HttpResponse::BadRequest().body("Computing the GCD with zero is boring.");
    }
    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",form.n,form.m,gcd(form.n,form.m));
    HttpResponse::Ok().body(response)
}

fn gcd(mut n:u64, mut m:u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0{
        if m < n{
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}