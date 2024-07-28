use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CalculatorInput {
    a: f64,
    b: f64,
    op: String,
}

#[derive(Serialize)]
struct CalculatorOutput {
    result: f64,
}

async fn calculate(info: web::Json<CalculatorInput>) -> impl Responder {
    let result = match info.op.as_str() {
        "+" => info.a + info.b,
        "-" => info.a - info.b,
        "*" => info.a * info.b,
        "/" => {
            if info.b == 0.0 {
                return HttpResponse::BadRequest().body("Division by zero");
            }
            info.a / info.b
        }
        _ => return HttpResponse::BadRequest().body("Invalid operator"),
    };

    HttpResponse::Ok().json(CalculatorOutput { result })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/calculate", web::post().to(calculate))
            .service(actix_web::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
