// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_allocate_hosts_input_input(input: &crate::input::AllocateHostsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AllocateHosts", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoPlacement");
    if let Some(var_2) = &input.auto_placement {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AvailabilityZone");
    if let Some(var_4) = &input.availability_zone {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ClientToken");
    if let Some(var_6) = &input.client_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("InstanceType");
    if let Some(var_8) = &input.instance_type {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("InstanceFamily");
    if let Some(var_10) = &input.instance_family {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Quantity");
    if let Some(var_12) = &input.quantity {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TagSpecification");
    if let Some(var_14) = &input.tag_specifications {
        let mut list_16 = scope_13.start_list(true, Some("item"));
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_17, item_15)?;
        }
        list_16.finish();
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("HostRecovery");
    if let Some(var_19) = &input.host_recovery {
        scope_18.string(var_19.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("OutpostArn");
    if let Some(var_21) = &input.outpost_arn {
        scope_20.string(var_21);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

