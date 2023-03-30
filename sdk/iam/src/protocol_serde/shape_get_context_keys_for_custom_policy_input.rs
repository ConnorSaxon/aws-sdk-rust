// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_context_keys_for_custom_policy_input_input(input: &crate::input::GetContextKeysForCustomPolicyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetContextKeysForCustomPolicy", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PolicyInputList");
    if let Some(var_2) = &input.policy_input_list {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

