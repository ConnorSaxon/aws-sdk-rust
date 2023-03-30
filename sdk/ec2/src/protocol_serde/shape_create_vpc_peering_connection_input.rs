// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_vpc_peering_connection_input_input(input: &crate::input::CreateVpcPeeringConnectionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateVpcPeeringConnection", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PeerOwnerId");
    if let Some(var_4) = &input.peer_owner_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PeerVpcId");
    if let Some(var_6) = &input.peer_vpc_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("VpcId");
    if let Some(var_8) = &input.vpc_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("PeerRegion");
    if let Some(var_10) = &input.peer_region {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("TagSpecification");
    if let Some(var_12) = &input.tag_specifications {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_15, item_13)?;
        }
        list_14.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

