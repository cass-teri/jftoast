use std::collections::HashMap;
use crate::options::Options;

#[derive(Debug)]
pub enum UiSchemaElement{
    Control{
        id: String,
        scope: String,
        options: Option<Options>
    },
    HorizontalLayout {
        id: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    VerticalLayout {
        id: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    Group {
        id: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    Categorization {
        id: String,
        label: String,
        categories: Option<Vec<UiSchemaElement>>,
        option: Option<HashMap<String, String>>
    },
    Category {
        id: String,
        label: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    HelpContent{
        id: String,
        label: Option<String>,
        options: Option<Options>,
        elements: Option<Vec<UiSchemaElement>>
    },
    GroupLayout{
        id: String,
        label: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    Unknown,
}