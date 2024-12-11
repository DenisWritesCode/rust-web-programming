use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    println!("logout endpoint called");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html>\
                <script>\
                    localStorage.removeItem('user-token');\
                    window.location.replace(document.location.origin);\
                </script>\
              </html>",
        )
}
