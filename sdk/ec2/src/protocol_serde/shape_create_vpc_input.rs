// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_vpc_input_input(input: &crate::input::CreateVpcInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateVpc", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CidrBlock");
    if let Some(var_2) = &input.cidr_block {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AmazonProvidedIpv6CidrBlock");
    if let Some(var_4) = &input.amazon_provided_ipv6_cidr_block {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Ipv6Pool");
    if let Some(var_6) = &input.ipv6_pool {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Ipv6CidrBlock");
    if let Some(var_8) = &input.ipv6_cidr_block {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Ipv4IpamPoolId");
    if let Some(var_10) = &input.ipv4_ipam_pool_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Ipv4NetmaskLength");
    if let Some(var_12) = &input.ipv4_netmask_length {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Ipv6IpamPoolId");
    if let Some(var_14) = &input.ipv6_ipam_pool_id {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Ipv6NetmaskLength");
    if let Some(var_16) = &input.ipv6_netmask_length {
        scope_15.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("DryRun");
    if let Some(var_18) = &input.dry_run {
        scope_17.boolean(*var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("InstanceTenancy");
    if let Some(var_20) = &input.instance_tenancy {
        scope_19.string(var_20.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Ipv6CidrBlockNetworkBorderGroup");
    if let Some(var_22) = &input.ipv6_cidr_block_network_border_group {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("TagSpecification");
    if let Some(var_24) = &input.tag_specifications {
        let mut list_26 = scope_23.start_list(true, Some("item"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_27, item_25)?;
        }
        list_26.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

