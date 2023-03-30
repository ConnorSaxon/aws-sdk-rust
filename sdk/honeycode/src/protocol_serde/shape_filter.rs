// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.formula {
        object.key("formula").string(var_1.as_str());
    }
    if let Some(var_2) = &input.context_row_id {
        object.key("contextRowId").string(var_2.as_str());
    }
    Ok(())
}

