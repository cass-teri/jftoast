mod schema;
mod json_7_schema;
mod help_content;
mod ui_schema_element;
mod parse_help;
mod ast;
mod schema_package;
mod categorization;
mod category;
mod control;
mod options;
mod component_props;

use std::fs;
use std::path::Path;
use serde_json::Value;
use serde_json::from_str;
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
                            None => UiSchemaElement::None,
                            Some(elements) => {
                                match elements {
                                    Value::Array(elements) => {
                                        let children : Vec<UiSchemaElement> = elements.iter().map(|e|{
                                            parse_node(e)
                                        }).collect();

                                        match s {
                                            "VerticalLayout" => UiSchemaElement::VerticalLayout{ id: gen_id(), elements: children},
                                            "HorizontalLayout" => UiSchemaElement::HorizontalLayout(children),
                                            "Group" => UiSchemaElement::Group(children),
                                            "Category"=> UiSchemaElement::Category(children),
                                            "Categorization" => UiSchemaElement::Categorization(children);
                                            _ => UiSchemaElement::None

                                        }
                                    }
                                    _ => UiSchemaElement::None
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
                                        UiSchemaElement::Control { id: gen_id(), scope: s.to_owned(), options: () }
                                    }
                                    _ => UiSchemaElement::None
                                }
                            }
                        }
                    }
                    "HelpContent"=>{
                        //UiSchemaElement::HelpContent(HelpContent::Header(s.to_owned()))
                        let help_content = parse_help::parse_help(json_data);
                        UiSchemaElement::HelpContent(help_content)
                    }
                    _ => UiSchemaElement::None
                }
            }
            _ => UiSchemaElement::None
        }
    } else {
        UiSchemaElement::None
    }
}

fn gen_id() -> String {
    "id".to_owned()
}

