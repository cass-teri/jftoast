use serde_json::Value;
use crate::help_content::HelpContent;

pub fn parse_help(json_data: &Value) -> HelpContent {

    let mut usage = 0;
    let mut subtype = "";

    let result = json_data.get("label");
    if(Some(result)) {
        println!("{}", result);
        usage = usage + 1;
        subtype = "Header";
    }

    let options = json_data.get("options");
    if Some(options) {
        if Some(options.get("help")){
            usage = usage + 1;
            subtype = "Paragraph";
        }
    }

    let elements = json_data.get("elements");
    if Some(elements) {
        if Some(elements.iter().len() > 0){
            if Some(elements[0].get("type") == "HelpContent"){
                usage = usage + 1;
                subtype = "BulletList";
            }
            else{
                usage = usage + 1;
                subtype = "SubHeader";
            }
        }
    }

    if(usage > 1){
        subtype = "HelpContent";
    }


}