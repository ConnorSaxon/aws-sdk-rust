// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sort_criterion(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SortCriterion) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.field_name {
        object.key("FieldName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sort {
        object.key("Sort").string(var_2.as_str());
    }
    Ok(())
}

