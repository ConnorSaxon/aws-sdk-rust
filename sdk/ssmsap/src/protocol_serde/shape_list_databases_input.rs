// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_databases_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDatabasesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_id {
        object.key("ApplicationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.component_id {
        object.key("ComponentId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

