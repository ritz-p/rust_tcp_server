use std::collections::HashMap;

fn get_parts_css(tag: String,content: String) -> String{
    tag + &String::from(" {\n") + &content + &String::from("}\n")
}



pub fn create_css(properties: HashMap<&str,Vec<(&str,&str)>>) -> String{
    let mut content = String::from("");
    for (tag,value) in properties.iter(){
        let mut style = String::from("");
        for propertie in value{
            let added_content = propertie.0.to_string() + &String::from(": ") + &propertie.1 + &String::from(";\n");
            style += &added_content;
        }
        content += &get_parts_css(tag.to_string(), style);
    }
    content
}