pub mod tag;
pub mod html_type;

pub struct Html<'a>{
    html_properties: Vec<(html_type::html::HtmlProperty,&'a str)>,
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct Head<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}