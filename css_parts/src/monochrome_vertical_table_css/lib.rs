use std::collections::HashMap;
use css_constant::create_css;

pub fn create_monochrome_vertical_table_css() -> String{
    let mut table:Vec<(&str,&str)> = vec![];
    table.push(("width","100%"));
    table.push(("border","solid 2px black"));
    table.push(("border-collapse","collapse"));

    let mut th: Vec<(&str,&str)> = vec![];
    th.push(("color","#fff"));
    th.push(("background-color","#222"));
    th.push(("border","dashed 1px white"));

    let mut td: Vec<(&str,&str)> = vec![];
    td.push(("color","#222"));
    td.push(("background-color","#fff"));
    td.push(("border","dashed 1px black"));
    td.push(("text-align","center"));
    

    let mut css_hash:HashMap<&str,Vec<(&str,&str)>> = HashMap::new();
    css_hash.insert("table",table);
    css_hash.insert("th",th);
    css_hash.insert("td",td);
    create_css(css_hash)
}