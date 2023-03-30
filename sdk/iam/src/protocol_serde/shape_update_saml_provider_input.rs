// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_saml_provider_input_input(input: &crate::input::UpdateSamlProviderInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UpdateSAMLProvider", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SAMLMetadataDocument");
    if let Some(var_2) = &input.saml_metadata_document {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SAMLProviderArn");
    if let Some(var_4) = &input.saml_provider_arn {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

