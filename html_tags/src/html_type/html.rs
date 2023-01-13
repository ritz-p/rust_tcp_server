pub enum HtmlProperty{
    Xmlns,
}

impl HtmlProperty {
    fn as_str(&self) -> &'static str{
        match self{
            HtmlProperty::Xmlns => "xmlns",
        }
    }
}