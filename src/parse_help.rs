use serde_json::Value;
use crate::ui_schema_element::UiSchemaElement;
use crate::ui_schema_element::UiSchemaElement::HelpContent;
use cuid2::create_id;
use crate::options::{Options, StringOrStrings};

pub fn parse_help(json_data: &Value) -> UiSchemaElement {

    let mut usage = 0;
    let mut subtype = "";
    let mut header= "";
    //let mut sub_header = "";
    let mut paragraph = "";
    //let mut bullet_list = vec![];
    let id_options = json_data.get("id");
    let id = match id_options {
        Some(id) => {
            id.as_str().unwrap().to_owned()
        }
        None => {
            create_id().as_str().to_owned()
        }
    };


    let result = json_data.get("label");
    if let Some(result) = result {
        header = result.as_str().unwrap();
        usage = usage + 1;
        subtype = "Header";
    }

    let options = json_data.get("options");
    if let Some(options) = options {
        let paragraph_option = options.get("help");
        if let Some(paragraph_option) = paragraph_option{
            paragraph = paragraph_option.as_str().unwrap();
            usage = usage + 1;
            subtype = "Paragraph";
        }
    }

    let elements = json_data.get("elements");
    if let Some(elements) = elements {
        //if Some(elements.iter().len() > 0){
        //    // TODO: Should be if sub HelpContent includes label
        //    if Some(elements[0].get("type") == "HelpContent"){
        //        usage = usage + 1;
        //        subtype = "BulletList";
        //        bullet_list = json_data.get("options").unwrap().get("help").unwrap()
        //    }
        //    else{
        //        sub_header = json_data.get("label");
        //        usage = usage + 1;
        //        subtype = "SubHeader";
        //    }
        //}
    }

    if usage > 1 {
        subtype = "HelpContent";
    }

    match subtype {
        "Header" => {
            HelpContent {
                id: id.to_owned(),
                label:Some(header.to_owned()),
                options: None,
                elements: None,
            }
        },
        "SubHeader" => {
            let label = json_data.get("label");
            let label = match label {
                Some(label) => {
                    label.as_str().unwrap().to_owned()
                }
                None => {
                    create_id().as_str().to_owned()
                }
            };
            let sub_content = HelpContent{
                id: id.to_owned(),
                label: Some(label.to_owned()),
                options: None,
                elements: None,
            };

            HelpContent {
                id: id.to_owned(),
                label: None,
                options: None,
                elements: Some(vec![sub_content]),
            }
        },
        "Paragraph" => {

            let mut options: Options = Default::default();
            options.help = Some(StringOrStrings::String(paragraph.to_owned()));

            HelpContent {
                id: id.to_owned(),
                label: None,
                options: Some(options),
                elements: None,
            }
        }
        // "BulletList" => {
        //     HelpContent::Bullets(bullet_list)
        // }
        _ => {
            let mut options: Options = Default::default();
            options.help = Some(StringOrStrings::String(paragraph.to_owned()));
            HelpContent{
                id: id.to_owned(),
                label: Some(header.to_owned()),
                options: Some(options),
                elements: None,
            }
        }
    }







}