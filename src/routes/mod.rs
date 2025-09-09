use actix_web::web;
pub mod handlers;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/tv", web::get().to(handlers::handle_get_tv_shows));
    cfg.route("/movies", web::get().to(handlers::handle_get_movies));
    cfg.route("/System/Info/Public", web::get().to(handlers::handle_get_system_info));
    cfg.route("/UserViews", web::get().to(handlers::handle_get_collections));
}