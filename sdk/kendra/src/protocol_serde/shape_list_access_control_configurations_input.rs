// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_access_control_configurations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAccessControlConfigurationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_id {
        object.key("IndexId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.next_token {
        object.key("NextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

