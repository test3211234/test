use actix_web::{post, web, App, HttpResponse, HttpServer, Result};
use image::GenericImageView;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ImageLink {
    link: String,
}

#[post("/submit")]
async fn submit(image_link: web::Json<ImageLink>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(&image_link.link)
        .await?
        .bytes()
        .await?;

    let img = image::load_from_memory(&bytes)?;

    let mut pixel_colors: Vec<[u8; 4]> = Vec::new();

    for pixel in img.pixels() {
        pixel_colors.push(pixel.2.0);
    }

    Ok(HttpResponse::Ok().json(
        json!({
            "width": img.width(),
            "height": img.height(),
            "pixelColors": pixel_colors,
        })
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(submit)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
