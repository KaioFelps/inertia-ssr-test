use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use inertia::{InertiaPage, InertiaAppResponse};

/// mock of a inertia render endpoint of the ssr server,
/// that processes a page component and returns
/// the rendered head elements and body html string
#[get("/render")]
async fn render(page: web::Json<Option<InertiaPage>>) -> impl Responder {
    let page = match page.into_inner() {
        None => return HttpResponse::BadRequest().finish(),
        Some(page) => page,
    };

    let ssr_app = InertiaAppResponse {
        head: vec![
            "<title inertia>Hello inertia!</title>".to_string(),
            "<description>Just a mocked head... Ha!</description>".to_string()
        ],
        body: format!("<div id='app' data-page='{}'", serde_json::to_string(&page.props).unwrap())
    };

    return HttpResponse::Ok().json(ssr_app);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(render)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}