use crate::component_props::ComponentProps;

#[derive(Debug, Default)]
pub enum StringOrStrings{
    #[default]
    String(String),
    Strings(Vec<String>)
}

#[derive(Debug, Default)]
pub struct Options{
    pub(crate) multi: Option<bool>,
    pub(crate) component_props: Option<ComponentProps>,
    pub(crate) radio: Option<bool>,
    pub(crate) text_for_true: Option<String>,
    pub(crate) text_for_false: Option<String>,
    pub(crate) format: Option<String>,
    pub(crate) help: Option<StringOrStrings>
}