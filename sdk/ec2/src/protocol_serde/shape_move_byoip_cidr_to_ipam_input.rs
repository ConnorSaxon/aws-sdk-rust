// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_move_byoip_cidr_to_ipam_input_input(input: &crate::input::MoveByoipCidrToIpamInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "MoveByoipCidrToIpam", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Cidr");
    if let Some(var_4) = &input.cidr {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("IpamPoolId");
    if let Some(var_6) = &input.ipam_pool_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("IpamPoolOwner");
    if let Some(var_8) = &input.ipam_pool_owner {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

