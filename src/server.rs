use actix_multipart::Multipart;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, web};
use futures::{StreamExt, TryStreamExt};
use image::ImageFormat;
use image::ImageReader;
use std::fs;
use std::io::Write;
use uuid::Uuid;

async fn index() -> impl Responder {
    let html = include_str!("../index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn convert_image_web(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let unique_id = Uuid::new_v4();
    let chemin_temp_entree = format!("./tmp/{}.input", unique_id);
    let mut target_format_str = "png".to_string();

    // Read the form
    while let Ok(Some(mut field)) = payload.try_next().await {
        let disposition = match field.content_disposition() {
            Some(d) => d,
            None => continue,
        };
        let name = disposition.get_name().unwrap_or("");

        if name == "format" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk?);
            }
            target_format_str = String::from_utf8(data).unwrap_or("png".to_string());
        } else if name == "fichier" {
            let mut f = fs::File::create(&chemin_temp_entree)?;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f.write_all(&data)?;
            }
        }
    }

    // Determine the target format
    let format_clean = target_format_str.trim().to_lowercase();
    let (target_format_enum, mime_type) = match format_clean.as_str() {
        "jpg" | "jpeg" => (ImageFormat::Jpeg, "image/jpeg"),
        "gif" => (ImageFormat::Gif, "image/gif"),
        "bmp" => (ImageFormat::Bmp, "image/bmp"),
        "ico" => (ImageFormat::Ico, "image/x-icon"),
        _ => (ImageFormat::Png, "image/png"),
    };

    let chemin_sortie = format!("./tmp/{}.{}", unique_id, format_clean);
    let path_in = chemin_temp_entree.clone();
    let path_out = chemin_sortie.clone();

    // Conversion
    let conversion_result = web::block(move || {
        if !std::path::Path::new(&path_in).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not received",
            ));
        }

        let img = ImageReader::open(&path_in)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
            .with_guessed_format()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
            .decode()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        img.save_with_format(&path_out, target_format_enum)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    })
    .await;

    // Cleanup
    if let Ok(Ok(_)) = conversion_result {
        let file_bytes = fs::read(&chemin_sortie)?;
        let _ = fs::remove_file(chemin_temp_entree);
        let _ = fs::remove_file(&chemin_sortie);
        Ok(HttpResponse::Ok().content_type(mime_type).body(file_bytes))
    } else {
        let _ = fs::remove_file(chemin_temp_entree);
        Ok(HttpResponse::InternalServerError().body("Error during conversion"))
    }
}

pub async fn run() -> std::io::Result<()> {
    fs::create_dir_all("./tmp")?;
    println!("🌍 Server mode active.");
    println!("👉 Open http://127.0.0.1:8080 in your browser");
    println!("(Press Ctrl+C to stop)");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(convert_image_web))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
