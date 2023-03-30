// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_certificate_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RevokeCertificateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate_serial {
        object.key("CertificateSerial").string(var_2.as_str());
    }
    if let Some(var_3) = &input.revocation_reason {
        object.key("RevocationReason").string(var_3.as_str());
    }
    Ok(())
}

