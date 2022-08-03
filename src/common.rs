/// 取得.env文件key里的值
pub fn get_env(key: &str) -> String {
    dotenv::dotenv().ok();
    let msg = ".env文件必须配置的环境变量：".to_string() + key;
    let value = std::env::var(key).expect(&msg);
    value
}

/* 打印变量与变量类型 */
pub fn type_v<T>(t: T)
where
    T: std::fmt::Debug,
{
    println!("变量值：{:?}  =>类型： {}", t, core::any::type_name::<T>());
}

/// 定义接口标准返回格式： response.
///
/// # 示例(Examples)
///
/// ```
/// # use warp::http::StatusCode;
///
/// let status_code = warp::http::StatusCode::OK;
/// let data = "data数据结构".to_string();
/// let message = "成功".to_string();
/// rresponse_json(status_code, Some(&data), None);
/// ```
pub fn response_json<T>(
    status: warp::http::StatusCode,
    data: Option<&T>,
    message: Option<String>,
) -> std::result::Result<impl warp::Reply, warp::Rejection>
where
    T: ?Sized + serde::Serialize,
{
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct RespondData<T>
    where
        T: Serialize,
    {
        status: u16,
        message: Option<String>,
        data: Option<T>,
    }

    let response = RespondData {
        status: status.as_u16(),
        message: message,
        data: data,
    };

    let response_string = serde_json::to_string(&response).unwrap().clone();

    Ok(warp::http::Response::builder()
        .status(status)
        .header("Content-type", "application/json")
        .body(response_string))
}

pub fn _response_json_old(
    status: warp::http::StatusCode,
    data: String,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct RespondData {
        status: u16,
        data: String, //message
    }

    let response = RespondData {
        status: status.as_u16(),
        data: data,
    };

    // let kd=serde_json::to_string(&response).unwrap();

    Ok(warp::http::Response::builder()
        .status(status)
        .header("Content-type", "application/json")
        .body(serde_json::to_string(&response).unwrap()))
}

pub fn _response_json_old_yl<T>(
    status: warp::http::StatusCode,
    data: &T,
) -> std::result::Result<impl warp::Reply, warp::Rejection>
where
    T: ?Sized + serde::Serialize,
{
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct RespondData<T>
    where
        T: ?Sized + Serialize,
    {
        status: u16,
        data: T, //message message: Option<String>,
    }

    let response = RespondData {
        status: status.as_u16(),
        data: data,
    };

    let response_string = serde_json::to_string(&response).unwrap().clone();

    Ok(warp::http::Response::builder()
        .status(status)
        .header("Content-type", "application/json")
        .body(response_string))
}

/* 产生随机字符串 */
pub fn random_key(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::thread_rng;
    use rand::Rng;
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(len)
        .collect()
}

pub fn now_naive_date_time() -> chrono::NaiveDateTime {
    // use chrono::prelude::{Local, NaiveDate, NaiveDateTime};
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = chrono::prelude::Local::now();
    let dft = now.format(fmt);
    let str_date = dft.to_string();
    // println!("当前时间：{}", str_date);
    let now_date_time =
        chrono::prelude::NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();
    // let now_date = chrono::prelude::NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

    return now_date_time;
}

pub fn page(path: &str, count: i64, page: u32, per: u32) -> String {
    let base_url = get_env("BASE_URL");
    let page_url = format!("{}{}", base_url, path);

    let count_page = ((count as f32) / (per as f32)).ceil() as u32; //总页数

    /*
    <a data-toggle="modal" data-target="#myModal" href="javascript:void(0)" class="m-left">
        <span class="fa fa-angle-left"></span>
    </a>
    <span class="m-active"><a href="javascript:void(0)">1</a></span>
    <span><a href="javascript:void(0)">2</a></span>
    <span class="m-active"><a href="javascript:void(0)">3</a></span>
    <span><a href="javascript:void(0)">4</a></span>

    <a href="javascript:void(0)" class="m-right">
        <span class="fa fa-angle-right"></span>
    </a>
     */
    let mut show_left = 2; //左边显示的数字数
    let mut show_right = 2; //右边显示的数字数
    let mut page_html = String::new();

    //首页
    if page > 2 {}

    // 是否有上一页
    if page > 1 {
        // 当前页的上一页,非最前字字上一页
        page_html = format!(
            r#"{}
            <a data-toggle="modal" href="{}/{}" class="m-left">
            <span class="fa fa-angle-left"></span>
            </a>
            "#,
            page_html,
            page_url,
            page - 1
        );

        // 左边的页数数字,如左边少页数,则在右边补
        if page - show_left <= 0 {
            show_right+=show_left; 
            show_left = page - 1;  
            show_right -= show_left;
        }
        while show_left > 0 {
            page_html = format!(
                r#"
            {}
            <span><a href="{}/{}">{2}</a></span>
            "#,
                page_html,
                page_url,
                page - show_left
            );
            show_left -= 1;
        }

        // 上一页数字
        // page_html = format!(
        //     r#"
        // {}
        // <span><a href="{}/{}">{2}</a></span>
        // "#,
        //     page_html,
        //     page_url,
        //     page - 1
        // );
    }

    // 当前页
    page_html = format!(
        r#"
    {}
    <span class="m-active"><a href="javascript:void(0)">{}</a></span>
    "#,
        page_html, page
    );

    //是否有下一页
    if page < count_page {
        // 下一页数字
        // page_html = format!(
        //     r#"
        // {}
        // <span><a href="{}/{}">{2}</a></span>
        // "#,
        //     page_html,
        //     page_url,
        //     page + 1
        // );

        // 输出右边数字
        let mut t = 1;
        loop {
            if page + show_right > count_page {
                show_right = count_page - page;
            }

            if t > show_right {
                break;
            }

            page_html = format!(
                r#"
            {}
            <span><a href="{}/{}">{2}</a></span>
            "#,
                page_html,
                page_url,
                page + t
            );

            t += 1;
            // show_right -= 1;
            // break;
        }

        // 下一页
        page_html = format!(
            r#"{}
            <a href="{}/{}" class="m-right">
                <span class="fa fa-angle-right"></span>
            </a>
            "#,
            page_html,
            page_url,
            page + 1
        );
    }

    //末页
    if page < count_page - 1 {}

    page_html
}

/*
<div class="b-items__pagination-main">


    &lt;span class="m-active"&gt;&lt;a href="javascript:void(0)"&gt;1&lt;/a&gt;&lt;/span&gt;
                        </div>
                         */
