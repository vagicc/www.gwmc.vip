use crate::models::lawsuit_autocar_model;
use crate::template::{to_html_base, to_html_single};
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
    let per: u32 = 18; //每页总数
    let (count, list) = lawsuit_autocar_model::get_list(Some(page), Some(per));
    let pages = crate::pager::default_full("lawsuit/autocar", count, page, per);

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(list.len()));
    data.insert("list".to_string(), to_json(list));
    data.insert("pages".to_string(), to_json(pages));

    data.insert("seo_title".to_string(), to_json("跟我买车-官网"));
    data.insert(
        "seo_keyword".to_string(),
        to_json("司法拍卖车;跟我买车;二手车;法拍车推荐;法拍车详细过户流程;最新法拍车"),
    );
    data.insert(
        "seo_description".to_string(),
        to_json("跟我买法拍车;买二手车不如来‘跟我买车’选购高性价比无坑的车;司法拍卖车如何购买"),
    );

    // let html = to_html_single("lawsuit_autocar_list.html", data);
    let html = to_html_base("lawsuit_autocar/list.html", data);
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

    let seo_title = detail.title.clone();
    let seo_keyword = format!(
        "法拍车推荐;法拍车详细过户流程;{:?};{:?}",
        detail.license.clone().unwrap_or("二手车".to_string()),
        detail
            .disposal_unit
            .clone()
            .unwrap_or("跟我买无坑的司法拍卖车".to_string())
    );
    let seo_description = detail.summary.clone();

    let mut data = Map::new();
    data.insert("detail".to_string(), to_json(detail));
    data.insert("article".to_string(), to_json(article));
    data.insert("photo".to_string(), to_json(photo));

    data.insert("seo_title".to_string(), to_json(&seo_title));
    data.insert("seo_keyword".to_string(), to_json(seo_keyword));
    data.insert("seo_description".to_string(), to_json(seo_description));

    // let html = to_html_single("lawsuit_autocar_detail.html", data);
    let html = to_html_base("lawsuit_autocar/detail.html", data);

    Ok(warp::reply::html(html))
}
