use crate::json_7_schema::JsonSchema7;
use crate::ui_schema_element::UiSchemaElement;

pub struct SchemaPackage{
    pub(crate) ui_schema: UiSchemaElement,
    pub(crate) data_schema: JsonSchema7,
    pub id: String,
    pub is_container: bool,
}