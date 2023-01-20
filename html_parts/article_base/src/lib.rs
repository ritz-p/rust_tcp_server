use html_tags::*;
use html_constant::*;
use html_tags::html_type::global::GlobalProperty;
use html_tags::html_type::meta::MetaProperty;
use table_base::diagonal_table;
use table_base::vertical_table;
use table_base::horizontal_table;
use title_base::basic_title;
use css_parts::monochrome_vertical_table_css::lib::create_monochrome_vertical_table_css;
use css_parts::monochrome_title_css::lib::create_monochrome_title_css;
pub fn create_article_base() -> String{
    let article_title = "テスト記事";
    let test:Vec<Vec<&str>> = vec![vec!["test","test","test"],["1","2","3"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec()];
    let test2:Vec<Vec<&str>> = vec![vec!["test","test","test"],["1","2","3"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec()];
    let test3:Vec<Vec<&str>> = vec![vec!["test","test","test"],["1","2","3"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec(),["adf","daf","adsfasd"].to_vec()];
    let headlines = vec!["こんにちは","ウンチ","adfadfadf"];
    let html_properties:HtmlProperties = HtmlProperties{
        html_properties: vec![],
        global_properties: vec![(GlobalProperty::Lang,"ja")]
    };
    let html = TagName::Html(html_properties);
    let html_tag = Tag{
        variable: html
    };

    let head_properties:HeadProperties = HeadProperties { global_properties: vec![] };
    let head = TagName::Head(head_properties);
    let head_tag = Tag{
        variable: head
    };

    let meta_properties:MetaProperties = MetaProperties { 
        meta_properties: vec![(MetaProperty::Charset,"utf-8")],
        global_properties: vec![]
    };
    let meta = TagName::Meta(meta_properties);
    let meta_tag = Tag{
        variable: meta
    };

    let title_properties:TitleProperties = TitleProperties { 
        global_properties: vec![]
    };
    let title = TagName::Title(title_properties);
    let title_tag = Tag{
        variable: title
    };

    let body_properties:BodyProperties = BodyProperties { 
        body_properties: vec![], global_properties: vec![] 
    };
    let body = TagName::Body(body_properties);
    let body = Tag{
        variable: body
    };


    PairTag::bind(&html_tag,
        PairTag::bind(&head_tag,
            SingleTag::bind(&meta_tag,false) + 
            &PairTag::bind(&title_tag," - rust blog".to_owned() 
        )
            // &PairTag::bind(&STYLE,
            //     create_monochrome_vertical_table_css() +
            //     &create_monochrome_title_css()
        ) +
        &PairTag::bind(&body,
            basic_title::lib::create_title_base(article_title, headlines)
            // &PairTag::bind(&P,"asdfasdfasdfadsfasdfadfadsf".to_string()) +
            // &SingleTag::bind(&BR,false) +
            // &PairTag::bind(&P,"adfasdfasdfadfa".to_string()) +
            // &vertical_table::lib::create_vertical_table(test) + 
            // &horizontal_table::lib::create_horizontal_table(test2) +
            // &diagonal_table::lib::create_diagonal_table(test3)
        )
    )
}