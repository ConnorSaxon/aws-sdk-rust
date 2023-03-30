// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_traffic_mirror_session_input_input(input: &crate::input::CreateTrafficMirrorSessionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateTrafficMirrorSession", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NetworkInterfaceId");
    if let Some(var_2) = &input.network_interface_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TrafficMirrorTargetId");
    if let Some(var_4) = &input.traffic_mirror_target_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TrafficMirrorFilterId");
    if let Some(var_6) = &input.traffic_mirror_filter_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PacketLength");
    if let Some(var_8) = &input.packet_length {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("SessionNumber");
    if let Some(var_10) = &input.session_number {
        scope_9.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("VirtualNetworkId");
    if let Some(var_12) = &input.virtual_network_id {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Description");
    if let Some(var_14) = &input.description {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("TagSpecification");
    if let Some(var_16) = &input.tag_specifications {
        let mut list_18 = scope_15.start_list(true, Some("item"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_19, item_17)?;
        }
        list_18.finish();
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("DryRun");
    if let Some(var_21) = &input.dry_run {
        scope_20.boolean(*var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("ClientToken");
    if let Some(var_23) = &input.client_token {
        scope_22.string(var_23);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

