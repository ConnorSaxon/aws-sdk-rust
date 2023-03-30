// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_services_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeServicesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_code {
        object.key("ServiceCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.format_version {
        object.key("FormatVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

