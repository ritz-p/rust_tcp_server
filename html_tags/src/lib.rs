pub mod tag;
pub mod html_type;

impl TagName<'_>{
    fn get_tag_name(&self) -> &str{
        match self {
            TagName::Html(_) => "html",
            TagName::Head(_) => "head",
            TagName::Meta(_) => "meta",
            TagName::Body(_) => "body",
            TagName::Title(_) => "title",
            TagName::Div(_) => "div",
            TagName::P(_) => "p",
            TagName::BR(_) => "br",
            TagName::Table(_) => "table",
            TagName::TBody(_) => "tbody",
            TagName::THead(_) => "thead",
            TagName::TFoot(_) => "tfoot",
            TagName::TH(_) => "th",
            TagName::TD(_) => "td",
            TagName::TR(_) => "tr",
            TagName::LI(_) => "li",
            TagName::UL(_) => "ul",
            TagName::OL(_) => "ol",
        }
    }
}
pub enum TagName<'a>{
    Html(HtmlProperties<'a>),
    Head(HeadProperties<'a>),
    Meta(MetaProperties<'a>),
    Body(BodyProperties<'a>),
    Title(TitleProperties<'a>),
    Div(DivProperties<'a>),
    P(PProperties<'a>),
    BR(BRProperties<'a>),
    Table(TableProperties<'a>),
    TBody(TBodyProperties<'a>),
    THead(THProperties<'a>),
    TFoot(TFootProperties<'a>),
    TH(THProperties<'a>),
    TD(TDProperties<'a>),
    TR(TRProperties<'a>),
    LI(LIProperties<'a>),
    UL(ULProperties<'a>),
    OL(OLProperties<'a>),
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
pub struct LIProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    li_properties: Vec<(html_type::li::LI,&'a str)>,
}

pub struct ULProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct OLProperties<'a>{
    global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    ol_properties: Vec<(html_type::ol::OL,&'a str)>
}