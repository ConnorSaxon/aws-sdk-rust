// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_certificate_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteCertificateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_name {
        object.key("certificateName").string(var_1.as_str());
    }
    Ok(())
}

