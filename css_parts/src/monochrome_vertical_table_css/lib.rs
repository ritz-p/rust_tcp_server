use std::collections::HashMap;
use css_constant::create_css;

pub fn create_monochrome_vertical_table_css() -> String{
    let mut table:Vec<(String,String)> = vec![];
    table.push(("border".to_string(),"solid 1px".to_string()));
    let mut css_hash:HashMap<String,Vec<(String,String)>> = HashMap::new();
    css_hash.insert("table".to_string(),table);
    create_css(css_hash)
}