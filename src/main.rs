use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, error,
    get, post, web, App, HttpResponse, HttpServer, Responder, Error, Either, guard
};
use std::{
    cell::Cell,
    sync::Mutex,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};
use serde::{Deserialize, Serialize};
use futures::{future::ok, stream::once};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn index_obj() -> impl Responder {
    MyObj { name: "user" }
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

struct AppState {
    app_name: String,
}

#[derive(Clone)]
struct AppStateClone {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/clone")]
async fn show_count_clone(data: web::Data<AppStateClone>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/clone/add")]
async fn add_one_clone(data: web::Data<AppStateClone>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/state")]
async fn index2(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index_mutable_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

async fn index_extraction(path: web::Path<(String, String)>, json: web::Json<Info>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.user_id, json.friend)
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index_path(path: web::Path<(u32, String)>) -> Result<String, Error> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

/// extract path info using serde
#[get("/users/serde/{user_id}/{friend}")] // <- define path parameters
async fn index_serde(info: web::Path<Info>) -> Result<String, Error> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

#[get("/users/match/{user_id}/{friend}")] // <- define path parameters
async fn index_matched(req: HttpRequest) -> Result<String, Error> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Deserialize)]
struct InfoUser {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/query")]
async fn index_query(info: web::Query<InfoUser>) -> String {
    format!("Welcome {}!", info.username)
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<InfoUser>) -> Result<String, Error> {
    Ok(format!("Welcome {}!", info.username))
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index_submit(info: web::Json<InfoUser>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn index_form(form: web::Form<FormData>) -> Result<String, Error> {
    Ok(format!("Welcome {}!", form.username))
}

#[derive(Clone)]
struct AppCountState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppCountState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppCountState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

async fn is_a_variant() -> bool {
    true
}


async fn index_either() -> RegisterResult {
    if is_a_variant().await {
        // choose Left variant
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello!"))
    }
}

#[get("/query")]
async fn show_users(info: web::Query<InfoUser>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });


    let data = AppCountState {
        count: Cell::new(0),
    };

    let data_clone = AppStateClone {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    

    HttpServer::new(move || {
        let scope = web::scope("/users").service(show_users);
        let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            // create custom error response
            error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
        });
        App::new()
            .service(scope)
            .service(stream)
            .app_data(web::Data::new(data_clone.clone()))
            .service(show_count_clone)
            .service(add_one_clone)


            .app_data(web::Data::new(data.clone()))
            .route("/count", web::to(show_count))
            .route("/count/add", web::to(add_one))
            .route("/obj", web::to(index_obj))
            .route("/either", web::to(index_either))
            
            
            .service(
                web::resource("/json")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(index))
            )
            .service(index_path)
            .service(index_serde)
            .service(index_matched)
            .service(index_query)
            .service(
                web::scope("/app")
                .route("/index.html", web::get().to(index))
            )
            .service(hello)
            .service(echo)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index2)
            
            .route("/submit", web::post().to(index_submit))
            .route("/hey", web::get().to(manual_hello))
            .route("/extraction", web::get().to(index_extraction))
            
            .app_data(counter.clone()) // <- register the created data
            .route("/mutableState", web::get().to(index_mutable_state))
            
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .route("/ok", web::to(HttpResponse::Ok))

            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/config",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}