// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_license_usage_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetLicenseUsageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.license_arn {
        object.key("LicenseArn").string(var_1.as_str());
    }
    Ok(())
}

