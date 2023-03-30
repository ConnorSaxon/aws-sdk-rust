// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_carrier_gateway_input_input(input: &crate::input::CreateCarrierGatewayInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateCarrierGateway", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VpcId");
    if let Some(var_2) = &input.vpc_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TagSpecification");
    if let Some(var_4) = &input.tag_specifications {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_7, item_5)?;
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("DryRun");
    if let Some(var_9) = &input.dry_run {
        scope_8.boolean(*var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("ClientToken");
    if let Some(var_11) = &input.client_token {
        scope_10.string(var_11);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

