use crate::component_props;
use crate::component_props::ComponentProps;
use crate::json_7_schema::JsonSchema7;
use crate::options::Options;
use crate::schema_package::SchemaPackage;
use crate::ui_schema_element::UiSchemaElement::*;

pub fn GetSchemasForComponentType(component_type: &str, id: &str) -> SchemaPackage{
    match (component_type) {
        "Group"=>{
            let ui_schema = GroupLayout{
                id: id.to_owned(),
                label: id.to_owned(),
                elements: None
            };
            let data_schema : JsonSchema7= Default::default();

            SchemaPackage {
                ui_schema,
                data_schema,
                id: id.to_owned(),
                is_container: true
            }
        }
        "HorizontalLayout"=> {
            let ui_schema = HorizontalLayout{
                id: id.to_owned(),
                elements: None
            };

            SchemaPackage {
                ui_schema,
                data_schema: Default::default(),
                id: id.to_owned(),
                is_container: true,
            }
        },
        "VerticalLayout"=> {
            let ui_schema = VerticalLayout{
                id: id.to_owned(),
                elements: None
            };
            let data_schema : JsonSchema7= Default::default();

            SchemaPackage {
                ui_schema,
                data_schema,
                id: id.to_owned(),
                is_container: true
            }
        }
        "Categorization"=> {
            let ui_schema = Categorization {
                label: id.to_string(),
                categories: None,
                option: None
            };

            SchemaPackage{
                ui_schema,
                data_schema: Default::default(),
                is_container: true,
                id: id.to_owned()
            }
        }
        "Category"=> {
            let ui_schema = Category {
                label: id.to_string(),
                elements: None,
            };

            SchemaPackage{
                ui_schema,
                data_schema: Default::default(),
                is_container: true,
                id: id.to_owned()
            }
        }
        "Text"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema : JsonSchema7  = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "string".to_owned();

            SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
        }
        "Number"=> {
            let ui_schema= Control{
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: None,
            };

            let mut data_schema:JsonSchema7 = Default::default();
            data_schema.id= format!("{id}");
            data_schema._type = "number".to_owned();

            SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
        }
        "Boolean"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(Options {
                    radio: true,
                    text_for_true: "Yes".to_owned(),
                    text_for_false: "No".to_owned(),
                })
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "boolean".to_owned();

            SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
        }
        "Integer"=>{
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "integer".to_owned();

            SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
        }
        "DropDown"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "string".to_owned();
            data_schema._enum = vec!["".to_owned()];

            SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
        }
        "Date"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema._type = "string".to_owned();
            data_schema.format = "date".to_owned();

            SchemaPackage { ui_schema, data_schema,  is_container: false, id: id.to_owned() }
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
            };

            let ui_schema= Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Some(options)
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "string".to_owned();

            SchemaPackage {
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
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
            data_schema.id = format!("{id}");
            data_schema._type = "string".to_owned();
            data_schema._enum = vec!["".to_owned()];

            SchemaPackage{
                ui_schema,
                data_schema,
                is_container: false,
                id: id.to_owned()
            }
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
            data_schema.id = format!("{id}");
            data_schema._type = "boolean".to_owned();

            SchemaPackage{ ui_schema, data_schema, is_container: false, id : id.to_owned()}
        }
        "PostalCode" => {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id= format!("{id}");
            data_schema._type= "string".to_owned();
            data_schema.pattern= "^[A-Za-z]\\d[A-Za-z][ -]?\\d[A-Za-z]\\d$".to_owned();

            SchemaPackage { ui_schema, data_schema, is_container: false, id: id.to_owned() }
        }
        "Email"=> {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format!("#/properties/{id}"),
                options: Default::default(),
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type= "string".to_owned();
            data_schema.pattern= "^.+@.+\\.[a-zA-Z]{2,}$".to_owned();

            SchemaPackage {ui_schema, data_schema, is_container: false, id: id.to_owned()}
        }

        "Phone" => {
            let ui_schema = Control {
                id: id.to_owned(),
                scope: format !("#/properties/{id}"),
                options: Default::default()
            };

            let mut data_schema: JsonSchema7 = Default::default();
            data_schema.id = format!("{id}");
            data_schema._type = "string".to_owned();
            data_schema.pattern = "^\\d{3}[ -]?\\d{3}[ -]?\\d{4}$".to_owned();

            SchemaPackage{ ui_schema, data_schema, is_container: false, id: id.to_owned() }
        }
        "Header"=> {
            let ui_schema = HelpContent {
                id: id.to_owned(),
                label: "Header".to_owned(),
                options: None,
                elements: None,
            };

            SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() }
        }
        "Paragraph"=> {
            let mut options: Options = Default::default();
            options.help = "Paragraph Text".to_owned();

            let ui_schema = HelpContent{
                id: id.to_owned(),
                label: Default::default(),
                options: Some(options),
                elements: None,
            };

            SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id: id.to_owned() }
        }
        "SubHeader"=> {
            let sub_content = HelpContent { id: id.to_owned(), label: "SubHeader".to_owned(), options: None, elements: None };
            let ui_schema= HelpContent {
                id: id.to_owned(),
                label: Default::default(),
                options: None,
                elements: Some(vec![ sub_content ])
            };

            SchemaPackage{ ui_schema, data_schema: Default::default(),  is_container: false, id.to_owned() }
        }
        "HelpContent"=> {
            let sub_content = HelpContent { id: id.to_owned(), label: "SubHeader".to_owned(), options: None, elements: None };
            let mut options :Options  = Default::default();
            options.help = "Paragraph".to_owned();

            let ui_schema = HelpContent {
                id: id.to_owned(),
                label: "Header".to_owned(),
                elements: Some(vec![ sub_content]),
                options: Some(options)
            };

            SchemaPackage{ ui_schema, data_schema: Default::default(), is_container: false, id : id.to_owned() }
        }

        "Bullets"=> {
            let sub_content = HelpContent {
                id: id.to_owned(),
                label: "".to_owned(),
                options: {
                    help: Some(vec!(["one".to_owned(), "two".to_owned(), "three".to_owned()]))
                },
                elements: None,
            };

            let ui_schema :HelpContent = Default::default();
            ui_schema.elements: Some(vec!([ sub_content ]));

            SchemaPackage { ui_schema, data_schema: Default::default(),  is_container: false, id: id.to_owned() }
        }
        "Details"=> {
            let ui_schema: HelpContent = {
                label: "Title: {id}",
                options: {
                    variant: "details",
                    help: "Text: {id}"
                }
            }

            SchemaPackage{ ui_schema, data_schema: {}, meta: { is_container: false, id } }
        }
        "Image"=>{
            let ui_schema: HelpContent = {
                options: {
                    variant: "img",
                    src: "",
                    alt: "",
                    height: "",
                    width: ""
                }
            }

            SchemaPackage{ ui_schema, data_schema: {}, meta: { is_container: false, id } }
        }
        "Link"=> {
            let ui_schema: HelpContent = {
                jtype: "HelpContent",
                options: {
                    variant: "hyperlink",
                    link: "http://alberta.ca",
                    help: "Link: {id}"
                }
            }

            SchemaPackage { ui_schema, data_schema: {}, meta: { is_container: false, id } }
        }
        "Repeater"=> {
            let ui_schema: Control = {
                jtype: "Control",
                scope: format!("#/properties/{id}")
            }

            let data_schema: JsonSchema7 = {
                [id]: {
                    items: {
                        jtype: "object",
                        properties: {}
                    }
                }
            }

            SchemaPackage{ ui_schema, data_schema, meta: { is_container: false, id } }
        }
        "Province"=> {
            let ui_schema: Control = {
                jtype: "Control",
                scope: format!("#/properties/{id}")
            }

            let data_schema: JsonSchema7 = {
                ["{id}"]: {
                    jtype: "string",
                    enum: [
                        "Alberta",
                        "British Columbia",
                        "Manitoba",
                        "New Brunswick",
                        "Newfoundland and Labrador",
                        "Nova Scotia",
                        "Northwest Territories",
                        "Nunavut",
                        "Ontario",
                        "Prince Edward Island",
                        "Quebec",
                        "Saskatchewan",
                        "Yukon"
                    ]
                }
            }

            SchemaPackage{ ui_schema, data_schema, meta: { is_container: false, id } }
        }
        "Ministry"=> {
            let ui_schema: Control = {
                jtype: "Control",
                scope: format!("#/properties/{id}")
            };

            let data_schema: JsonSchema7 = {
                ["{id}"]: {
                    jtype: "string",
                    enum: [
                        "Agriculture and Forestry",
                        "Children's Services",
                        "Community and Social Services",
                        "Culture and Multiculturalism",
                        "Economic Development, Trade and Tourism",
                        "Education",
                        "Energy",
                        "Environment and Parks",
                        "Health",
                        "Indigenous Relations",
                        "Infrastructure",
                        "Justice and Solicitor General",
                        "Labour",
                        "Municipal Affairs",
                        "Seniors and Housing",
                        "Service Alberta",
                        "Transportation",
                        "Treasury Board and Finance"
                    ]
                }
            }
            SchemaPackage{ ui_schema, data_schema, meta: { is_container: false, id } }
        }

        default: {
        return { ui_schema: {}, data_schema: {}, meta: { is_container: false, id } } as SchemaPackage
        }
    }
