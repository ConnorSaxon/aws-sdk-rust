// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_failures_for_license_configuration_operations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFailuresForLicenseConfigurationOperationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.license_configuration_arn {
        object.key("LicenseConfigurationArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    Ok(())
}

