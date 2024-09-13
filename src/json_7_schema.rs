use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct JsonSchema7 {
    _ref: Option<String>,
    /////////////////////////////////////////////////
    // Schema Metadata
    /////////////////////////////////////////////////
    /**
     * This is important because it tells refs where
     * the root of the document is located
     */
    pub(crate) id: Option<String>,
    /**
     * It is recommended that the meta-schema is
     * included in the root of any JSON Schema
     */
    schema: Option<String>,
    /**
     * Title of the schema
     */
    title: Option<String>,
    /**
     * Schema description
     */
    description: Option<String>,
    /**
     * Default json for the object represented by
     * this schema
     */
    default: Option<String>,

    /////////////////////////////////////////////////
    // Number Validation
    /////////////////////////////////////////////////
    /**
     * The value must be a multiple of the number
     * (e.g. 10 is a multiple of 5)
     */
    multiple_of: Option<u32>,
    maximum: Option<i32>,
    /**
     * If true maximum must be > value, >= otherwise
     */
    exclusive_maximum: Option<i32>,
    minimum: Option<i32>,
    /**
     * If true minimum must be < value, <= otherwise
     */
    exclusive_minimum: Option<i32>,

    /////////////////////////////////////////////////
    // String Validation
    /////////////////////////////////////////////////
    max_length: Option<i32>,
    min_length: Option<i32>,
    /**
     * This is a regex String that the value must
     * conform to
     */
    pub(crate) pattern: Option<String>,

    /////////////////////////////////////////////////
    // Array Validation
    /////////////////////////////////////////////////
    additional_items: Option<bool>,
    items: Option<Vec<Box<JsonSchema7>>>,
    max_items: Option<i32>,
    min_items: Option<i32>,
    unique_items: Option<bool>,

    /////////////////////////////////////////////////
    // Object Validation
    /////////////////////////////////////////////////
    max_properties: Option<i32>,
    min_properties: Option<i32>,
    required: Option<Vec<String>>,
    additional_properties: Option<bool> ,
    /**
     * Holds simple JSON Schema definitions for
     * referencing from elsewhere.
     */
    definitions: Option<HashMap<String, Box<JsonSchema7>>>,
    /**
     * The keys that can exist on the object with the
     * json schema that should validate their value
     */
    properties: Option<HashMap<String, Box<JsonSchema7>>>,
    /**
     * The key of this object is a regex for which
     * properties the schema applies to
     */
    pattern_properties: Option<HashMap<String, Box<JsonSchema7>>>,
    /**
     * If the key is present as a property then the
     * String of properties must also be present.
     * If the value is a JSON Schema then it must
     * also be valid for the object if the key is
     * present.
     */
    dependencies: Option<HashMap<String, Box<JsonSchema7>>>,

    /////////////////////////////////////////////////
    // Generic
    /////////////////////////////////////////////////
    /**
     * Enumerates the values that this schema can be
     * e.g.
     * {"type": "String",
     *  "enum": ["red", "green", "blue"]}
     */
    pub(crate) _enum: Option<Vec<String>>,
    /**
     * The basic type of this schema, can be one of
     * [String, number, object, array, bool, null]
     * or an array of the acceptable types
     */
    pub(crate) _type: Option<String>,

    /////////////////////////////////////////////////
    // Combining Schemas
    /////////////////////////////////////////////////
    all_of: Option<Vec<Box<JsonSchema7>>>,
    any_of: Option<Vec<Box<JsonSchema7>>>,
    one_of: Option<Vec<Box<JsonSchema7>>>,
    /**
     * The entity being validated must not match this schema
     */
    not: Option<Box<JsonSchema7>>,

    pub(crate) format: Option<String>,
    read_only:          Option<bool>,
    write_only:         Option<bool>,
    examples:           Option<Vec<String>>,
    contains:           Option<Box<JsonSchema7>>,
    property_names:     Option<Box<JsonSchema7>>,
    _const:             Option<Vec<String>>,
    _if:                Option<Box<JsonSchema7>>,
    then:               Option<Box<JsonSchema7>>,
    _else:              Option<Box<JsonSchema7>>,
    error_message:      Option<Vec<String>>
}

