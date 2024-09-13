use crate::component_props::ComponentProps;
use crate::json_7_schema::JsonSchema7;
use crate::options::{Options, StringOrStrings};
use crate::schema_package::SchemaPackage;
use crate::ui_schema_element::UiSchemaElement;
use crate::ui_schema_element::UiSchemaElement::*;
use crate::UiSchemaElement::HelpContent;

pub fn get_schemas_for_component_type(component_type: &str, id: &str) -> Option<SchemaPackage>{
    match component_type {
        "Group"=>{
            let ui_schema = GroupLayout{
                id: id.to_owned(),
                label: format!("{}",id),
                elements: None
            };
            let data_schema : JsonSchema7= Default::default();

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                id: id.to_owned(),
                is_container: true
            })
        }
        "HorizontalLayout"=> {
            let ui_schema = HorizontalLayout{
                id: id.to_owned(),
                elements: None
            };

            Some(SchemaPackage {
                ui_schema,
                data_schema: Default::default(),
                id: id.to_owned(),
                is_container: true,
            })
        },
        "VerticalLayout"=> {
            let ui_schema = VerticalLayout{
                id: id.to_owned(),
                elements: None
            };
            let data_schema : JsonSchema7= Default::default();

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                id: id.to_owned(),
                is_container: true
            })
        }
        "Categorization"=> {
            let ui_schema = Categorization {
                id: id.to_owned(),
                label: id.to_string(),
                categories: None,
                option: None
            };

            Some(SchemaPackage{
                ui_schema,
                data_schema: Default::default(),
                is_container: true,
                id: id.to_owned()
            })
        }
        "Category"=> {
            let ui_schema = Category {
                id: id.to_owned(),
                label: id.to_string(),
                elements: None,
            };

            Some(SchemaPackage{
                ui_schema,
                data_schema: Default::default(),
                is_container: true,
                id: id.to_owned()
            })
        }
        "Text"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema : JsonSchema7  = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());

            Some(SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Number"=> {
            let ui_schema= Control{
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema:JsonSchema7 = Default::default();
            data_schema.id= Some(format!("{id}"));
            data_schema._type = Some("number".to_owned());

            Some(SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Boolean"=> {
            let options = Options {
                multi: None,
                component_props: None,
                radio: Some(true),
                text_for_true: Some("Yes".to_owned()),
                text_for_false: Some("No".to_owned()),
                format: None,
                help: None,
                variant: None,
                src: None,
                alt: None,
                height: None,
                width: None,
                link: None,
            };
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(options)
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("boolean".to_owned());

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Integer"=>{
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("integer".to_owned());

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "DropDown"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());
            data_schema._enum = Some(vec!["".to_owned()]);

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Date"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema._type = Some("string".to_owned());
            data_schema.format = Some("date".to_owned());

            Some(SchemaPackage { ui_schema, data_schema,  is_container: false, id: id.to_owned() })
        }
        "Textarea"=> {
            let component_props = ComponentProps{
                rows: 4
            };

            let options = Options {
                multi: Some(true),
                component_props: Some(component_props),
                radio: None,
                text_for_true: None,
                text_for_false: None,
                format: None,
                help: None,
                variant: None,
                src: None,
                alt: None,
                height: None,
                width: None,
                link: None,
            };

            let ui_schema= Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(options)
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());

            Some(SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Radio"=> {
            let mut options = Options::default();
            options.format = Some("radio".to_owned());
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(options)
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());
            data_schema._enum = Some(vec!["".to_owned()]);

            Some(SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            })
        }
        "Checkbox"=> {
            let mut options : Options  = Default::default();
            options.format = Some("checkbox".to_owned());

            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(options)
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("boolean".to_owned());

            Some(SchemaPackage{ ui_schema, data_schema, is_container: false, id: id.to_owned() })
        }
        "PostalCode" => {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id= Some(format!("{id}"));
            data_schema._type= Some("string".to_owned());
            data_schema.pattern= Some("^[A-Za-z]\\d[A-Za-z][ -]?\\d[A-Za-z]\\d$".to_owned());

            Some(SchemaPackage { ui_schema, data_schema, is_container: false, id: id.to_owned() })
        }
        "Email"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type= Some("string".to_owned());
            data_schema.pattern= Some("^.+@.+\\.[a-zA-Z]{2,}$".to_owned());

            Some(SchemaPackage {ui_schema, data_schema, is_container: false, id: id.to_owned() })
        }

        "Phone" => {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format !("#/properties/{id}"),
                options: Default::default()
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());
            data_schema.pattern = Some("^\\d{3}[ -]?\\d{3}[ -]?\\d{4}$".to_owned());

            Some(SchemaPackage{ ui_schema, data_schema, is_container: false, id: id.to_owned() })
        }
        "Header"=> {
            let ui_schema = UiSchemaElement::HelpContent {
                id: id.to_owned(),
                label: Some("Header".to_owned()),
                options: None,
                elements: None,
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() })
        }
        "Paragraph"=> {
            let mut options: Options = Default::default();
            options.help = Some(StringOrStrings::String("Paragraph Text".to_owned()));

            let ui_schema = HelpContent{
                id: id.to_owned(),
                label: Default::default(),
                options: Some(options),
                elements: None,
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() })
        }
        "SubHeader"=> {
            let sub_content = HelpContent { id: id.to_owned(), label: Some("SubHeader".to_owned()), options: None, elements: None };
            let ui_schema= HelpContent {
                id: id.to_owned(),
                label: Default::default(),
                options: None,
                elements: Some(vec![ sub_content ])
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(),  is_container: false, id: id.to_owned() })
        }
        "HelpContent"=> {
            let sub_content = HelpContent { id: id.to_owned(), label: Some("SubHeader".to_owned()), options: None, elements: None };
            let mut options :Options  = Default::default();
            options.help = Some(StringOrStrings::String("Paragraph".to_owned()));

            let ui_schema = HelpContent {
                id: id.to_owned(),
                label: Some("Header".to_owned()),
                elements: Some(vec![ sub_content]),
                options: Some(options)
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() })
        }

        "Bullets"=> {
            let options = Options{
                multi: None,
                component_props: None,
                radio: None,
                text_for_true: None,
                text_for_false: None,
                format: None,
                help: Some(StringOrStrings::Strings(vec!["one".to_owned(), "two".to_owned(), "three".to_owned()])),
                           variant: None,
                           src: None,
                           alt: None,
                           height: None,
                           width: None,
                           link: None,
            };

            let sub_content = HelpContent {
                id: id.to_owned(),
                label: Some("".to_owned()),
                options: Some(options),
                elements: None,
            };

            let ui_schema = HelpContent {
                id: id.to_owned(),
                label: Some("".to_owned()),
                options: None,
                elements: Some(vec![ sub_content ])
            };

            Some(SchemaPackage { ui_schema, data_schema: Default::default(),  is_container: false, id: id.to_owned() })
        }
        "Details"=> {

            let options = Options {
                multi: None,
                component_props: None,
                radio: None,
                text_for_true: None,
                text_for_false: None,
                variant: Some("details".to_owned()),
                src: None,
                alt: None,
                height: None,
                width: None,
                help: Some(StringOrStrings::String(format!("Text: {id}"))),
                format: None,
                link: None,
            };
            let ui_schema = HelpContent{
                id: id.to_owned(),
                label: Some(format!("Title: {id}")),
                options: Some(options),
                elements: None,
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() })
        }
        "Image"=>{
            let options = Options {
                multi: None,
                component_props: None,
                radio: None,
                text_for_true: None,
                text_for_false: None,
                format: None,
                help: None,
                variant: Some("img".to_owned()),
                src: Default::default(),
                alt: Default::default(),
                height: Default::default(),
                width: Default::default(),
                link: None,
            };
            let ui_schema = HelpContent {
                id: id.to_owned(),
                label: Some("".to_owned()),
                options: Some(options),
                elements: None,
            };

            Some(SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() })
        }
        "Link"=> {
            let options = Options {
                multi: None,
                component_props: None,
                radio: None,
                text_for_true: None,
                text_for_false: None,
                variant: Some("hyperlink".to_owned()),
                src: None,
                alt: None,
                height: None,
                link: Some("http://alberta.ca".to_owned()),
                help: Some(StringOrStrings::String(format!("Link: {id}"))),
                format: None,
                width: None
            };

            let ui_schema = UiSchemaElement::HelpContent {
                id: id.to_owned(),
                label: None,
                options: Some(options),
                elements: None,
            };

            Some(SchemaPackage { ui_schema, data_schema: Default::default(),  is_container: false, id: id.to_owned() })
        }
        "Province"=> {
            let ui_schema = Control  {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type= Some("string".to_owned());
            data_schema._enum=  Some(vec![
                    "Alberta".to_owned(),
                    "British Columbia".to_owned(),
                    "Manitoba".to_owned(),
                    "New Brunswick".to_owned(),
                    "Newfoundland and Labrador".to_owned(),
                    "Nova Scotia".to_owned(),
                    "Northwest Territories".to_owned(),
                    "Nunavut".to_owned(),
                    "Ontario".to_owned(),
                    "Prince Edward Island".to_owned(),
                    "Quebec".to_owned(),
                    "Saskatchewan".to_owned(),
                    "Yukon".to_owned()
                ]);

            Some(SchemaPackage{ ui_schema, data_schema,  is_container: false, id: id.to_owned() })
        }
        "Ministry"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = Some(format!("{id}"));
            data_schema._type = Some("string".to_owned());
            data_schema._enum = Some(vec![
                "Agriculture and Forestry".to_owned(),
                "Children's Services".to_owned(),
                "Community and Social Services".to_owned(),
                "Culture and Multiculturalism".to_owned(),
                "Economic Development, Trade and Tourism".to_owned(),
                "Education".to_owned(),
                "Energy".to_owned(),
                "Environment and Parks".to_owned(),
                "Health".to_owned(),
                "Indigenous Relations".to_owned(),
                "Infrastructure".to_owned(),
                "Justice and Solicitor General".to_owned(),
                "Labour".to_owned(),
                "Municipal Affairs".to_owned(),
                "Seniors and Housing".to_owned(),
                "Service Alberta".to_owned(),
                "Transportation".to_owned(),
                "Treasury Board and Finance".to_owned()
            ]);

            Some(SchemaPackage{ ui_schema, data_schema,  is_container: false, id: id.to_owned() })
        }

        _=> {
            None
        }
    }
}