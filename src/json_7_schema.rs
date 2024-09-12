use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct JsonSchema7 {
    _ref: String,
    /////////////////////////////////////////////////
    // Schema Metadata
    /////////////////////////////////////////////////
    /**
     * This is important because it tells refs where
     * the root of the document is located
     */
    pub(crate) id: String,
    /**
     * It is recommended that the meta-schema is
     * included in the root of any JSON Schema
     */
    schema: String,
    /**
     * Title of the schema
     */
    title: String,
    /**
     * Schema description
     */
    description: String,
    /**
     * Default json for the object represented by
     * this schema
     */
    default: String,

    /////////////////////////////////////////////////
    // Number Validation
    /////////////////////////////////////////////////
    /**
     * The value must be a multiple of the number
     * (e.g. 10 is a multiple of 5)
     */
    multiple_of: u32,
    maximum: i32,
    /**
     * If true maximum must be > value, >= otherwise
     */
    exclusive_maximum: i32,
    minimum: i32,
    /**
     * If true minimum must be < value, <= otherwise
     */
    exclusive_minimum: i32,

    /////////////////////////////////////////////////
    // String Validation
    /////////////////////////////////////////////////
    max_length: i32,
    min_length: i32,
    /**
     * This is a regex String that the value must
     * conform to
     */
    pub(crate) pattern: String,

    /////////////////////////////////////////////////
    // Array Validation
    /////////////////////////////////////////////////
    additional_items: bool,
    items: Vec<JsonSchema7>,
    max_items: i32,
    min_items: i32,
    unique_items: bool,

    /////////////////////////////////////////////////
    // Object Validation
    /////////////////////////////////////////////////
    max_properties: i32,
    min_properties: i32,
    required: Vec<String>,
    additional_properties: bool ,
    /**
     * Holds simple JSON Schema definitions for
     * referencing from elsewhere.
     */
    definitions: HashMap<String, JsonSchema7>,
    /**
     * The keys that can exist on the object with the
     * json schema that should validate their value
     */
    properties: HashMap<String, JsonSchema7>,
    /**
     * The key of this object is a regex for which
     * properties the schema applies to
     */
    pattern_properties: HashMap<String, JsonSchema7>,
    /**
     * If the key is present as a property then the
     * String of properties must also be present.
     * If the value is a JSON Schema then it must
     * also be valid for the object if the key is
     * present.
     */
    dependencies: HashMap<String, JsonSchema7>,

    /////////////////////////////////////////////////
    // Generic
    /////////////////////////////////////////////////
    /**
     * Enumerates the values that this schema can be
     * e.g.
     * {"type": "String",
     *  "enum": ["red", "green", "blue"]}
     */
    pub(crate) _enum: Vec<String>,
    /**
     * The basic type of this schema, can be one of
     * [String, number, object, array, bool, null]
     * or an array of the acceptable types
     */
    pub(crate) _type: String,

    /////////////////////////////////////////////////
    // Combining Schemas
    /////////////////////////////////////////////////
    all_of: Vec<JsonSchema7>,
    any_of: Vec<JsonSchema7>,
    one_of: Vec<JsonSchema7>,
    /**
     * The entity being validated must not match this schema
     */
    not: JsonSchema7,

    pub(crate) format: String,
    read_only: bool,
    write_only: bool,
    examples: Vec<String>,
    contains: JsonSchema7,
    property_names: JsonSchema7,
    _const: Vec<String>,
    _if: JsonSchema7,
    then: JsonSchema7,
    _else: JsonSchema7,
    error_message: Vec<String>
}

