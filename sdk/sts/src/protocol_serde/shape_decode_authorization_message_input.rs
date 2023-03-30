// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_decode_authorization_message_input_input(input: &crate::input::DecodeAuthorizationMessageInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DecodeAuthorizationMessage", "2011-06-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EncodedMessage");
    if let Some(var_2) = &input.encoded_message {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

