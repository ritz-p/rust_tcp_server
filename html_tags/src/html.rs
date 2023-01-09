pub struct Tag<'a>{
    pub name: &'a str
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

pub trait TableTag{
    fn get_table(&self,content:String) -> String;
    fn get_head(&self,content:String) -> String;
    fn get_body(&self,content:String) -> String;
    fn get_tr(&self,content:String) -> String;
    fn get_th(&self,content:String) -> String;
    fn get_td(&self,content:String) -> String;
    fn bind(&self,content: Vec<Vec<&str>>)->String;
}

impl SingleTag for Tag<'_>{
    fn get_tag(&self) -> String {
        self.name.to_string()
    }
    fn bind(&self,content: String,use_end_slash: bool) -> String{
        let mut ret = String::from("<") + &self.name + &content;
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
        String::from("<") + &self.name + &String::from(">")
    }
    fn get_end_tag(&self)->String{
        String::from("</")+&self.name+&String::from(">")
    }
    fn bind(&self,content:String)->String{
        self.get_start_tag() + &content + &self.get_end_tag()
    }
}