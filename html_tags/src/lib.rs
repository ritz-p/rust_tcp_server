use html_type::PropertyInfo;

pub mod tag;
pub mod html_type;
pub mod css_type;
pub struct Tag<'a>{
    pub variable: TagName<'a>,
}

pub trait SingleTag{
    fn get_tag(&self) -> String;
    fn bind(&self,content:String,use_end_slash:bool) -> String;
}

pub trait PairTag{
    fn get_start_tag(&self) -> String;
    fn get_end_tag(&self) -> String;
    fn bind(&self,content: String) -> String;
}

pub trait Properties{
    fn get_properties(&self) -> String;
}

impl SingleTag for Tag<'_>{
    fn get_tag(&self) -> String {
        self.variable.get_tag_name().to_owned()
    }
    fn bind(&self,property: String,use_end_slash: bool) -> String{
        let mut ret = String::from("<") + &self.variable.get_tag_name().to_owned() + &property;
        if use_end_slash{
            ret = ret + &String::from("/>")
        }else{
            ret = ret + &String::from(">")
        }
        ret
    }
}

impl PairTag for Tag<'_>{
    fn get_start_tag(&self)->String{
        String::from("<") + &self.variable.get_tag_name().to_owned() + &String::from(">")
    }
    fn get_end_tag(&self)->String{
        String::from("</")+ &self.variable.get_tag_name().to_owned() + &String::from(">")
    }
    fn bind(&self,content:String)->String{
        self.get_start_tag() + &content + &self.get_end_tag()
    }
}

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

impl Properties for HtmlProperties<'_>{
    fn get_properties(&self) -> String{
        let mut res = "".to_owned();
        for (key,value) in &self.html_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        for (key,value) in &self.global_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        res
    }
}

pub struct HeadProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

impl Properties for HeadProperties<'_>{
    fn get_properties(&self) -> String {
        let mut res = "".to_owned();
        for (key,value) in &self.global_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        res
    }
}

pub struct MetaProperties<'a>{
    pub meta_properties: Vec<(html_type::meta::MetaProperty,&'a str)>,
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

impl Properties for MetaProperties<'_>{
    fn get_properties(&self) -> String {
        let mut res = "".to_owned();
        for (key,value) in &self.meta_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        for (key,value) in &self.global_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        res
    }
}

pub struct BodyProperties<'a>{
    pub body_properties: Vec<(html_type::body::BodyProperty,&'a str)>,
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

impl Properties for BodyProperties<'_>{
    fn get_properties(&self) -> String {
        let mut res = "".to_owned();
        for (key,value) in &self.body_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        for (key,value) in &self.global_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        res
    }
}

pub struct TitleProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

impl Properties for TitleProperties<'_>{
    fn get_properties(&self) -> String {
        let mut res = "".to_owned();
        for (key,value) in &self.global_properties{
            res += &(" ".to_owned() + key.as_str() + value);
        }
        res
    }
}

pub struct DivProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct BRProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct PProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TableProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TBodyProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct THeadProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TFootProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct TRProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct THProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    pub th_properties: Vec<(html_type::th::TH,&'a str)>
}

pub struct TDProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    pub td_properties: Vec<(html_type::td::TD,&'a str)>
}
pub struct LIProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    pub li_properties: Vec<(html_type::li::LI,&'a str)>,
}

pub struct ULProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>
}

pub struct OLProperties<'a>{
    pub global_properties: Vec<(html_type::global::GlobalProperty,&'a str)>,
    pub ol_properties: Vec<(html_type::ol::OL,&'a str)>
}