// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_domain_endpoint_options_input_input(input: &crate::input::UpdateDomainEndpointOptionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UpdateDomainEndpointOptions", "2013-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DomainName");
    if let Some(var_2) = &input.domain_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DomainEndpointOptions");
    if let Some(var_4) = &input.domain_endpoint_options {
        crate::protocol_serde::shape_domain_endpoint_options::ser_domain_endpoint_options(scope_3, var_4)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

