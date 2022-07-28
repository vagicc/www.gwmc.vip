use warp::Filter;

use crate::routes::home_route;
use crate::routes::lawsuit_autocar_route;

pub fn all_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let home = home_route::index();
    let lawsuit_autocar = lawsuit_autocar_route::list();
    // let demo = demo_route::all();

    let routes = home.or(dir).or(favicon).or(lawsuit_autocar);
    routes
}
