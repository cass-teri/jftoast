use crate::schema_package::SchemaPackage;

struct Ast {
    id: String,
    schema_package: SchemaPackage,
    children: Option<Vec<Ast>>,
    parent: Option(Box<Ast>),
    jtype: String
}