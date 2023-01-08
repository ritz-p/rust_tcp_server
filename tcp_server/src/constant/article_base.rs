use html_tags::html::{PairTag, SingleTag, TableTag};
use super::const_tags::*;

pub fn create_article_base() -> String{
    let mut test:Vec<Vec<&str>> = vec![vec!["test","test","test"],["1","2","3"].to_vec()];
    PairTag::bind(&HTML,
        PairTag::bind(&HEAD,
            SingleTag::bind(&META," charset=utf-8".to_string(),false) + 
            &PairTag::bind(&TITLE,"Test!!!!!!".to_string())
        ) + 
        &PairTag::bind(&BODY,
            PairTag::bind(&P,"asdfasdfasdfadsfasdfadfadsf".to_string()) +
            &SingleTag::bind(&BR,"".to_string(),false) +
            &PairTag::bind(&P,"adfasdfasdfadfa".to_string()) +
            &TableTag::bind(&TABLE,test)
        )
    )
}