use html_tags::tag::{PairTag};
use html_constant::*;

pub fn create_horizontal_table(content: Vec<Vec<&str>>) -> String{
    let mut head = String::from("");
    let mut body = String::from("");

    for i in 0..content[0].len(){
        if i == 0{
            head += &PairTag::bind(&TH,content[0][i].to_string());
        }else{
            head += &PairTag::bind(&TD,content[0][i].to_string());
        }
    }
    head = PairTag::bind(&TR,head);
    
    for i in 1..content.len(){
        let mut body_data = String::from("");
        for j in 0..content[i].len(){
            if j == 0{
                body_data += &PairTag::bind(&TH, content[i][j].to_string());
            }else{
                body_data += &PairTag::bind(&TD, content[i][j].to_string());
            }
        }
        body_data = PairTag::bind(&TR, body_data);
        body += &body_data;
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