use html_tags::html::{PairTag, SingleTag};
// use const_tags;

use super::const_tags::*;
pub fn create_article_base() -> String{
    PairTag::bind(&HTML,
        PairTag::bind(&HEAD,
            SingleTag::bind(&META," charset=utf-8".to_string(),false) + 
            &PairTag::bind(&TITLE,"Test!!!!!!".to_string())
        ) + 
        &PairTag::bind(&BODY,
            PairTag::bind(&P,"asdfasdfasdfadsfasdfadfadsf".to_string()) +
            &SingleTag::bind(&BR,"".to_string(),false) +
            &PairTag::bind(&P,"adfasdfasdfadfa".to_string())
        )
    )
}