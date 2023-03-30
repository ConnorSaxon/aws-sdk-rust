// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_container_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListContainerInstancesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter {
        object.key("filter").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.status {
        object.key("status").string(var_5.as_str());
    }
    Ok(())
}

