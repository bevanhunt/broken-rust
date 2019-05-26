extern crate calamine;
extern crate actix_web;
extern crate futures;
extern crate actix_multipart;
extern crate uuid;

#[macro_use(concat_string)]
extern crate concat_string;

use calamine::{Reader, Xlsx, open_workbook};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::cell::Cell;
use std::fs;
use std::io::Write;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr::NonNull;

// This is a self-referential struct since the slice field points to the data field.
// We cannot inform the compiler about that with a normal reference,
// since this pattern cannot be described with the usual borrowing rules.
// Instead we use a raw pointer, though one which is known to not be null,
// since we know it's pointing at the string.
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

pub struct AppState {
    pub counter: Cell<usize>,
}

pub fn save_file(field: Field, s: std::pin::Pin<std::boxed::Box<Unmovable>>) -> impl Future<Item = usize, Error = Error>  {
    let p = std::path::Path::new(&s.data);
    let file = match fs::File::create(p) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold(file, move |mut file, bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    Ok(file)
                })
                .map_err(|e: error::BlockingError<MultipartError>| {
                    match e {
                        error::BlockingError::Error(e) => e,
                        error::BlockingError::Canceled => MultipartError::Incomplete,
                    }
                })
            })
            .map(move |_| {
                let mut excel: Xlsx<_> = open_workbook(p).unwrap();
                if let Some(Ok(r)) = excel.worksheet_range("sheet1") {
                    return r.rows().len();
                }
                return 0;
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    multipart: Multipart,
) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| {
            let mut m = uuid::Uuid::encode_buffer();
            let b = uuid::Uuid::new_v4().to_hyphenated().encode_lower(&mut m);
            let a = String::from("./");
            let s = concat_string!(a, b);
            let unmoved = Unmovable::new(s);
            save_file(field, unmoved).into_stream()
        })
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    HttpServer::new(|| {
        App::new()
            .data(Cell::new(0usize))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to_async(upload)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
