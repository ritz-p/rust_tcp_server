use html_tags::{PairTag,SingleTag};
use super::const_tags::*;
fn create_vertical_table(content: Vec<Vec<&str>>) -> String{
    let mut head = String::from("");
    let mut body = String::from("");

    for head_data in content[0]{
        head += PairTag::bind(&TH,head_data);
    }
    head = PairTag::bind(&TR,head);
    
    for i in 1..content.len(){
        for body_data in content[i]{
            body += PairTag::bind(&TD,body_data);
        }
        body = PairTag::bind(&TR,body);
    }

    let ret = PairTag::bind(&TABLE,
        PairTag::bind(&THEAD,
            head
        )+
        PairTag.bind(&TBODY,
            body
        )
    );
    ret
}