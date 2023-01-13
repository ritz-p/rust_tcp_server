pub mod tag;
pub mod html_type;


pub enum TagName{
    Html,
    Head,
    Meta,
    Body,
    Title,
    Div,
    P,
    BR,
    Table,
    TBody,
    THead,
    TFoot,
    TH,
    TD,
    TR,
    LI,
    UL,
    OL,
}
pub struct HtmlProperties<'a>{
    pub html_properties: Vec<(html_type::html::HtmlProperty,&'a str)>,
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct HeadProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct MetaProperties<'a>{
    global_properties: Vec<(html_type::meta::MetaProperty,&'a str)>
}

pub struct BodyProperties<'a>{
    body_properties: Vec<(html_type::body::BodyProperty,&'a str)>,
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TitleProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct DivProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct BRProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct PProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TableProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TBodyProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct THeadProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TFootProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TRProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct THProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    th_properties: Vec<(html_type::th::TH,&'a str)>
}

pub struct TDProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    td_properties: Vec<(html_type::td::TD,&'a str)>
}
pub struct LI<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    li_properties: Vec<(html_type::li::LI,&'a str)>,
}

pub struct UL<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct OL<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    ol_properties: Vec<(html_type::ol::OL,&'a str)>
}