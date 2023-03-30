// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_certificate_from_csr_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateCertificateFromCsrInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_signing_request {
        object.key("certificateSigningRequest").string(var_1.as_str());
    }
    Ok(())
}

