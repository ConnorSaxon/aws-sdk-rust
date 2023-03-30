// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_internet_gateway_input_input(input: &crate::input::CreateInternetGatewayInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateInternetGateway", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TagSpecification");
    if let Some(var_2) = &input.tag_specifications {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DryRun");
    if let Some(var_7) = &input.dry_run {
        scope_6.boolean(*var_7);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

