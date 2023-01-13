use html_tags::tag::{PairTag};
use html_constant::*;

pub fn create_title_base(title_content: &str,headlines: Vec<&str>) -> String{
    let mut content = String::from("");
    content += &PairTag::bind(&H1,title_content.to_owned());
    let mut list = String::from("");
    for headline in headlines{
        list += &PairTag::bind(&LI,headline.to_owned());
    }
    list = PairTag::bind(&UL,list);
    content += &list;
    content
}