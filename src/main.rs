use std::fs;
use std::path::Path;
use serde_json::Value;
use serde_json::error::Category;
use serde_json::from_str;

fn main() {


    let file_path = Path::new("./ui_schema.json");
    let json_string = fs::read_to_string(file_path).unwrap();
    let json_data : Value= from_str(&json_string).unwrap();

    let result = parse_node(&json_data);
    println!("{:#?}", result)

}

#[derive(Debug)]
enum HelpContent{
    Header(String),
    SubHeader(String),
    Paragraph(String),
    Disclosure{},
    Bullets(Vec<String>)

}

#[derive(Debug)]
enum UiSchemaElement{
    Control{ scope: String},
    HorizontalLayout(Vec<UiSchemaElement>),
    VerticalLayout(Vec<UiSchemaElement>),
    Group(Vec<UiSchemaElement>),
    Categorization(Vec<Category>),
    Category(Vec<UiSchemaElement>),
    HelpContent(HelpContent),
    Unknown,
    None
}

fn parse_node(json_data: &Value) -> UiSchemaElement {
    if let Some(jtype) = json_data.get("type") {
        match jtype {
            Value::String(s) => {
                let s = s.as_str();
                match s {
                    "VerticalLayout" | "HorizontalLayout" | "Group" | "Categorization" | "Category" => {
                        let elements = json_data.get("elements");
                        match elements{
                            None => UiSchemaElement::None,
                            Some(elements) => {
                                match elements {
                                    Value::Array(elements) => {
                                        let children : Vec<UiSchemaElement> = elements.iter().map(|e|{
                                            parse_node(e)
                                        }).collect();

                                        match s {
                                            "VerticalLayout" => UiSchemaElement::VerticalLayout(children),
                                            "HorizontalLayout" => UiSchemaElement::HorizontalLayout(children),
                                            "Group" => UiSchemaElement::Group(children),
                                            "Category"=> UiSchemaElement::Category(children),
                                            //"Categorization" => UiSchemaElement::Categorization(children);
                                            _ => UiSchemaElement::Unknown

                                        }
                                    }
                                    _ => UiSchemaElement::Unknown
                                }
                            }
                        }
                    }
                    "Control" => {
                        match json_data.get("scope") {
                            None => panic!("Control Must Have Scope: {}", s),
                            Some(scope) => {
                                match scope {
                                    Value::String(s) => {
                                        UiSchemaElement::Control { scope: s.to_owned() }
                                    }
                                    _ => UiSchemaElement::Unknown
                                }
                            }
                        }
                    }
                    "HelpContent"=>{
                        UiSchemaElement::HelpContent(HelpContent::Header(s.to_owned()))
                    }
                    _ => UiSchemaElement::Unknown
                }
            }
            _ => UiSchemaElement::Unknown
        }
    } else {
        UiSchemaElement::Unknown
    }
}
