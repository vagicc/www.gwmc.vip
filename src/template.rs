use handlebars::{to_json, Handlebars};
use serde_json;
use serde_json::value::{Map, Value as Json};

/* 基础版嵌入式页 */
pub fn to_html_base(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame_base.html", "src/views/frame_base.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame_base.html")); //必传,这个是插入父级的html
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );

    //加上导航栏
    // let navbar = crate::models::navbar_model::show_navbar();
    let navbar = get_navbar_cache();
    data.insert("navbar".to_string(), to_json(navbar));

    let html = handlebars.render(name, &data).unwrap();
    html
}

pub fn get_navbar_cache() -> Vec<crate::models::navbar_model::Navbar> {
    use crate::models::navbar_model;
    use std::fs;

    let cache_file = "target/navbar.json";

    match fs::read_to_string(cache_file) {
        Ok(conter) => serde_json::from_str::<Vec<navbar_model::Navbar>>(&conter)
            .expect("导航栏缓存文件转结构体出错"),
        Err(e) => {
            log::warn!("读取导航栏缓存文件出错：{:#?}", e);
            let navbar = navbar_model::show_navbar();
            // 再写入缓存文件
            use std::fs::File;
            use std::io::Write;
            let mut output = File::create(cache_file).expect("创建导航栏缓存文件出失败");
            // let serialized = serde_json::to_string(&navbar).unwrap(); //转换为json字符
            // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
            write!(
                output,
                "{}",
                serde_json::to_string(&navbar).expect("结构体转为json字符出错")
            )
            .unwrap();
            navbar
        }
    }
}

/* 单页 */
pub fn to_html_single(tpl_name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板文件 */
    handlebars
        .register_template_file(tpl_name, "src/views/".to_owned() + tpl_name)
        .expect("handlebars注册模板出错");

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );
    let html = handlebars.render(tpl_name, &data).expect("注册模板出错");
    html
}
