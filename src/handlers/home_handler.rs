use warp::{Rejection, Reply};
use handlebars::{to_json, Handlebars};
use crate::template::to_html_single;
use serde_json::value::Map;

type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 响应/请求的返回 */
pub async fn index() -> ResultWarp<impl Reply> {
    // log::debug!("[调试信息]访问了“/”");
    // log::warn!("[警告信息] warn");
    // log::info!("[提示信息] info");
    
    let mut data = Map::new();

    let html = to_html_single("home.html", data);

    Ok(warp::reply::html(html))    //直接返回html
    // Err(warp::reject::not_found())   //错误的返回
}
