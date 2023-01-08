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

impl TableTag for Tag<'_>{
    fn get_table(&self,content:String) -> String {
        String::from("<table>") + &content + &String::from("</table>")
    }
    fn get_head(&self,content:String) -> String {
        String::from("<thead>") + &content + &String::from("</thead>")
    }
    fn get_body(&self,content:String) -> String {
        String::from("<tbody>") + &content + &String::from("</tbody>")
    }
    fn get_tr(&self,content:String) -> String {
        String::from("<tr>") + &content + &String::from("</tr>")
    }
    fn get_th(&self,content:String) -> String {
        String::from("<th>") + &content + &String::from("</th>")
    } 
    fn get_td(&self,content:String) -> String {
        String::from("<td>") + &content + &String::from("</td>")
    }
    fn bind(&self,content: Vec<Vec<&str>>)->String {
        if content.len() <= 1{
            return String::from("This Table is incorrect");
        }
        let mut head_tr = String::from("");
        let mut body_td = String::from("");
        for head in &content[0]{
            head_tr += &self.get_th(head.to_string());
        }
        head_tr = self.get_tr(head_tr);

        for i in 1..content.len(){
            for body in &content[i]{
                body_td += &self.get_td(body.to_string());
            }
            body_td = self.get_tr(body_td);
        }
        self.get_table(self.get_head(head_tr)+&self.get_body(body_td))
    }
}