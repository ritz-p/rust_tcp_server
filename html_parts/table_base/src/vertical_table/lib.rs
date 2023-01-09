use html_tags::html::{PairTag};
use html_constant::*;

pub fn create_vertical_table(content: Vec<Vec<&str>>) -> String{
    let mut head = String::from("");
    let mut body = String::from("");

    for head_data in &content[0]{
        head += &PairTag::bind(&TH,head_data.to_string());
    }
    head = PairTag::bind(&TR,head);
    
    for i in 1..content.len(){
        let mut body_td = String::from("");
        for body_data in &content[i]{
            body_td += &PairTag::bind(&TD,body_data.to_string());
        }
        body_td = PairTag::bind(&TR, body_td);
        body += &body_td;
    }

    let ret = PairTag::bind(&TABLE,
        PairTag::bind(&THEAD,
            head
        )+
        &PairTag::bind(&TBODY,
            body
        )
    );
    ret
}