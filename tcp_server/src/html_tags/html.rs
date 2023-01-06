pub struct Tag<'a>{
    pub name: &'a str
}

pub trait SingleTag{
    fn get_tag(&self) -> String;
}

pub trait PairTag{
    fn get_start_tag(&self) -> String;
    fn get_end_tag(&self) -> String;
    fn bind(&self,content: String) -> String;
}

impl SingleTag for Tag<'_>{
    fn get_tag(&self) -> String {
        self.name.to_string()
    }
}

impl PairTag for Tag<'_>{
    fn get_start_tag(&self)->String{
        String::from("<") + &self.name + &String::from(">")
    }
    fn get_end_tag(&self)->String{
        String::from("</")+&self.name+&String::from(">")
    }
    fn bind(&self,content:String)->String{
        self.get_start_tag() + &content + &self.get_end_tag()
    }
}