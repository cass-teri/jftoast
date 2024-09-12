use crate::ui_schema_element::UiSchemaElement;

pub struct Category {
    pub jtype: &'static str,
    pub label: String,
    pub elements: Vec<UiSchemaElement>
}