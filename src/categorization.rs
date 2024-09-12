use std::collections::HashMap;
use crate::ui_schema_element::UiSchemaElement;

pub struct Categorization {
    pub jtype: &'static str,
    pub label: String,
    pub elements: Vec<UiSchemaElement>,
    pub options: HashMap<String, String>
}