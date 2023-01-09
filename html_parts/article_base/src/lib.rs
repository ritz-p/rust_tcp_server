use html_tags::html::{PairTag, SingleTag};
use html_constant::*;
use table_base::vertical_table;
pub fn create_article_base() -> String{
    let test:Vec<Vec<&str>> = vec![vec!["test","test","test"],["1","2","3"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec()];
    PairTag::bind(&HTML,
        PairTag::bind(&HEAD,
            SingleTag::bind(&META," charset=utf-8".to_string(),false) + 
            &PairTag::bind(&TITLE,"Test!!!!!!".to_string())
        ) + 
        &PairTag::bind(&BODY,
            PairTag::bind(&P,"asdfasdfasdfadsfasdfadfadsf".to_string()) +
            &SingleTag::bind(&BR,"".to_string(),false) +
            &PairTag::bind(&P,"adfasdfasdfadfa".to_string()) +
            &vertical_table::lib::create_vertical_table(test)
        )
    )
}