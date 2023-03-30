// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_verified_email_address_input_input(input: &crate::input::DeleteVerifiedEmailAddressInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteVerifiedEmailAddress", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EmailAddress");
    if let Some(var_2) = &input.email_address {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

