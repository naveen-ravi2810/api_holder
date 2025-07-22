use actix_web::web;
mod users;

pub fn config(conf: &mut web::ServiceConfig) {
    let public_scope = web::scope("/api/v1")
        .service(web::resource("/register").route(web::post().to(users::register_handler)));
    let private_scope =
        web::scope("/api/v1").service(web::resource("/me").route(web::get().to(users::me_handler)));
    conf.service(public_scope).service(private_scope);
}
