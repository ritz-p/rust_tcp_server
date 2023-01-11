use std::collections::HashMap;

fn get_parts_css(tag: String,content: String) -> String{
    tag + &String::from(" {\n") + &content + &String::from("}\n")
}



pub fn create_css(properties: HashMap<String,Vec<(String,String)>>) -> String{
    let mut content = String::from("");
    for (tag,value) in properties.iter(){
        for propertie in value{
            let added_content = propertie.0.to_string() + &String::from(": ") + &propertie.1.to_string() + &String::from(";\n");
            content += &added_content;
        }
        content = get_parts_css(tag.to_string(), content);
    }
    content
}