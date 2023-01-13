pub enum OL{
    Reserved,
    Start,
    Type,
}

impl OL{
    fn as_str(&self) -> &'static str{
        match self{
            OL::Reserved => "reserved",
            OL::Start => "start",
            OL::Type => "type",
        }
    }
}