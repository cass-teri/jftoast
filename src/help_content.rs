#[derive(Debug)]
pub enum HelpContent{
    Header(String),
    SubHeader(String),
    Paragraph(String),
    Disclosure{},
    Bullets(Vec<String>)

}