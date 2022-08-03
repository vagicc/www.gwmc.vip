use crate::models::lawsuit_autocar_model;
use crate::template::to_html_single;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

// 直接写在路由上了
// pub async fn list_no_page() -> ResultWarp<impl Reply> {
//     list_page(1).await
// }

// 分页显示
pub async fn list(page: u32) -> ResultWarp<impl Reply> {
    let per: u32 = 3; //每页总数
    let (count, list) = lawsuit_autocar_model::get_list(Some(page), Some(per));
    let pages = crate::common::page("lawsuit/autocar", count, page, per);
    
    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(list.len()));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));

    let html = to_html_single("lawsuit_autocar_list.html", data);
    Ok(warp::reply::html(html))
}

pub async fn detail(id: i32) -> ResultWarp<impl Reply> {
    let detail = lawsuit_autocar_model::get_id(id);

    if detail.is_none() {
        log::warn!("查无此数据:lawsuit_autocar表无ID:{}", id);
        return Err(warp::reject::not_found()); //错误的返回
    }

    use crate::models::lawsuit_autocar_article_model;
    use crate::models::lawsuit_autocar_photo_model;

    let detail = detail.unwrap();
    let article = lawsuit_autocar_article_model::get_article(id);
    let photo = lawsuit_autocar_photo_model::get_autocar_photo(id);

    let mut data = Map::new();
    data.insert("detail".to_string(), to_json(detail));
    data.insert("article".to_string(), to_json(article));
    data.insert("photo".to_string(), to_json(photo));

    let html = to_html_single("lawsuit_autocar_detail.html", data);
    Ok(warp::reply::html(html))
}
