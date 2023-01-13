pub enum TH{
    Abbr,
    Headers,
    RowSpan,
    Scope,
    ColSpan,
}

impl TH{
    fn as_str(&self) ->&'static str{
        match self{
            TH::Abbr => "abbr",
            TH::Headers => "headers",
            TH::RowSpan => "rowspan",
            TH::Scope => "scope",
            TH::ColSpan => "colspan",
        }
    }
}