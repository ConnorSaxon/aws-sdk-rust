// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_stack_input_input(input: &crate::input::DeleteStackInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteStack", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StackName");
    if let Some(var_2) = &input.stack_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RetainResources");
    if let Some(var_4) = &input.retain_resources {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("RoleARN");
    if let Some(var_9) = &input.role_arn {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("ClientRequestToken");
    if let Some(var_11) = &input.client_request_token {
        scope_10.string(var_11);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

