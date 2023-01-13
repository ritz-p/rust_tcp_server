pub enum TD{
    ColSpan,
    RowSpan,
    Headers,
}

impl TD{
    fn as_str(&self) -> &'static str{
        match self{
            TD::ColSpan => "colspan",
            TD::RowSpan => "rowspan",
            TD::Headers => "headers",
        }
    }
}