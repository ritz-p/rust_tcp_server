pub enum MetaProperty{
    Charset,
    HttpEquiv,
    Content,
}

impl MetaProperty{
    fn as_str(&self) -> &'static str{
        match self{
            MetaProperty::Charset => "charset",
            MetaProperty::HttpEquiv => "http-equiv",
            MetaProperty::Content => "content",
        }
    }
}