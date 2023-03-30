// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_provision_byoip_cidr_input_input(input: &crate::input::ProvisionByoipCidrInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ProvisionByoipCidr", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Cidr");
    if let Some(var_2) = &input.cidr {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CidrAuthorizationContext");
    if let Some(var_4) = &input.cidr_authorization_context {
        crate::protocol_serde::shape_cidr_authorization_context::ser_cidr_authorization_context(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PubliclyAdvertisable");
    if let Some(var_6) = &input.publicly_advertisable {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Description");
    if let Some(var_8) = &input.description {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("PoolTagSpecification");
    if let Some(var_12) = &input.pool_tag_specifications {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_15, item_13)?;
        }
        list_14.finish();
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("MultiRegion");
    if let Some(var_17) = &input.multi_region {
        scope_16.boolean(*var_17);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

