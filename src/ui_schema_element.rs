use std::collections::HashMap;
use serde_json::error::Category;
use crate::help_content::HelpContent;
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
        label: String,
        categories: Option<Vec<Category>>,
        option: Option<HashMap<String, String>>
    },
    Category {
        label: String,
        elements: Option<Vec<UiSchemaElement>>
    },
    HelpContent{
        id: String,
        label: String,
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