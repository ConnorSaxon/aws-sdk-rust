// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_federation_token_input_input(input: &crate::input::GetFederationTokenInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetFederationToken", "2011-06-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    if let Some(var_2) = &input.name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Policy");
    if let Some(var_4) = &input.policy {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PolicyArns");
    if let Some(var_6) = &input.policy_arns {
        let mut list_8 = scope_5.start_list(false, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_policy_descriptor_type::ser_policy_descriptor_type(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("DurationSeconds");
    if let Some(var_11) = &input.duration_seconds {
        scope_10.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_11).into()));
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("Tags");
    if let Some(var_13) = &input.tags {
        let mut list_15 = scope_12.start_list(false, None);
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_16, item_14)?;
        }
        list_15.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

