pub enum LI{
    Value,
}

impl LI {
    fn as_str(&self) -> &'static str{
        match self{
            LI::Value => "li",
        }
    }
}