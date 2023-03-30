// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resend_validation_email_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ResendValidationEmailInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("CertificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain {
        object.key("Domain").string(var_2.as_str());
    }
    if let Some(var_3) = &input.validation_domain {
        object.key("ValidationDomain").string(var_3.as_str());
    }
    Ok(())
}

