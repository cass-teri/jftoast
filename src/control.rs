use std::collections::HashMap;

pub struct ControlElement {
    pub jtype: String,
    pub scope: String,
    pub options: HashMap<String, String>,
    pub id: String,
}