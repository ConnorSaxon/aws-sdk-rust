// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_source_identifier_to_subscription_input_input(input: &crate::input::AddSourceIdentifierToSubscriptionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AddSourceIdentifierToSubscription", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SubscriptionName");
    if let Some(var_2) = &input.subscription_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SourceIdentifier");
    if let Some(var_4) = &input.source_identifier {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

