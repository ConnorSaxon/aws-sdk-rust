// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dhcp_options_input_input(input: &crate::input::CreateDhcpOptionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateDhcpOptions", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DhcpConfiguration");
    if let Some(var_2) = &input.dhcp_configurations {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_new_dhcp_configuration::ser_new_dhcp_configuration(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("TagSpecification");
    if let Some(var_7) = &input.tag_specifications {
        let mut list_9 = scope_6.start_list(true, Some("item"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_10, item_8)?;
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("DryRun");
    if let Some(var_12) = &input.dry_run {
        scope_11.boolean(*var_12);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

