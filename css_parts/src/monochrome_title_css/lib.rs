use std::collections::HashMap;
use css_constant::create_css;

pub fn create_monochrome_title_css() -> String{
    let mut h1:Vec<(&str,&str)> = vec![];
    h1.push(("border-bottom","solid 3px black"));

    let mut ul:Vec<(&str,&str)> = vec![];
    ul.push(("border-bottom","dashed 1px black"));
    ul.push(("padding","0.5em 0 0.5em 1.5em"));

    let mut li:Vec<(&str,&str)> = vec![];
    li.push(("line-height","1.5"));
    li.push(("padding","0.5em 0"));
    
    let mut css_hash:HashMap<&str,Vec<(&str,&str)>> = HashMap::new();
    css_hash.insert("h1",h1);
    css_hash.insert("ul",ul);
    css_hash.insert("li", li);
    create_css(css_hash)
}