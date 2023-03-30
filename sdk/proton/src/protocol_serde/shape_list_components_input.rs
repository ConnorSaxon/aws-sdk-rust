// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_components_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListComponentsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("nextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.environment_name {
        object.key("environmentName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.service_name {
        object.key("serviceName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.service_instance_name {
        object.key("serviceInstanceName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    Ok(())
}

