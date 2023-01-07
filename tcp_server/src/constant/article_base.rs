use super::html_tags::html::PairTag;
use html_tags::html::SingleTag;
use const_tags;
fn create_article_base() -> String{
    PairTag::bind(HTML,
        PairTag::bind(HEAD,
            SingleTag::bind(META,"charset=utf-8",false) + 
            PairTag::bind(TITLE,"Test!!!!!!")
        ) + 
        PairTag::bind(BODY,
            PairTag::bind(P,"asdfasdfasdfadsfasdfadfadsf") +
            SingleTag::bind(BR,"",true) +
            PairTag::bind(P,"adfasdfasdfadfa")
        )
    )
}