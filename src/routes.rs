pub mod health_check;

use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::{get, post, HttpResponse, Responder};

use crate::security;

#[get("/")]
pub async fn home(user: security::auth::user::User) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", user))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("/photos")]
pub async fn post_photos(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let UploadForm { mut file } = form;
    let mut file_bytes = vec![0u8; 10 * 1024 * 1024];
    let i = file.file.read(&mut file_bytes).unwrap();

    // let img2 = ImageReader::new(Cursor::new(file_bytes)).with_guessed_format().unwrap().decode().unwrap();
    // format!("Uploaded file with size: {}. Content-type: {:?}. Filename: {:?}",
    //         file.size,
    //         file.content_type,
    //         file.file_name)

    // Restituisci i byte dell'immagine come flusso nella risposta
    HttpResponse::Ok()
        .content_type(ContentType::png()) // O il tipo di contenuto corretto
        .insert_header(ContentDisposition::attachment("image.png"))
        .body(file_bytes)
}
