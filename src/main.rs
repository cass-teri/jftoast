mod schema;
mod json_7_schema;
mod help_content;
mod ui_schema_element;
mod parse_help;
mod ast;
mod schema_package;
mod options;
mod component_props;

use serde_json::from_str;
use serde_json::Value;
use std::fs;
use std::path::Path;
use ui_schema_element::UiSchemaElement;

fn main() {

    let file_path = Path::new("./ui_schema.json");
    let json_string = fs::read_to_string(file_path).unwrap();
    let json_data : Value= from_str(&json_string).unwrap();

    let result = parse_node(&json_data);
    println!("{:#?}", result)

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
                            None => UiSchemaElement::Unknown,
                            Some(elements) => {
                                match elements {
                                    Value::Array(elements) => {
                                        let children : Vec<UiSchemaElement> = elements.iter().map(|e|{
                                            parse_node(e)
                                        }).collect();

                                        match s {
                                            "VerticalLayout" => UiSchemaElement::VerticalLayout{ id: gen_id(), elements: Some(children)},
                                            "HorizontalLayout" => UiSchemaElement::HorizontalLayout{ id: gen_id(), elements: Some(children)},
                                            "Group" => UiSchemaElement::Group{ id: gen_id(),  elements:Some(children)},
                                            "Category"=> UiSchemaElement::Category{ id: gen_id(), label: gen_id(),  elements:Some(children)},
                                            "Categorization" => UiSchemaElement::Categorization{ id: gen_id(), label: gen_id(), categories: Some(children),  option: None },
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
                                        UiSchemaElement::Control { id: gen_id(), scope: s.to_owned(), options: Default::default() }
                                    }
                                    _ => UiSchemaElement::Unknown
                                }
                            }
                        }
                    }
                    "HelpContent"=>{
                        //UiSchemaElement::HelpContent(HelpContent::Header(s.to_owned()))
                        parse_help::parse_help(json_data)
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

fn gen_id() -> String {
    cuid2::create_id()
}

