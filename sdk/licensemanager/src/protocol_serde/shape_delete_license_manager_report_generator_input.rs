// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_license_manager_report_generator_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteLicenseManagerReportGeneratorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.license_manager_report_generator_arn {
        object.key("LicenseManagerReportGeneratorArn").string(var_1.as_str());
    }
    Ok(())
}

