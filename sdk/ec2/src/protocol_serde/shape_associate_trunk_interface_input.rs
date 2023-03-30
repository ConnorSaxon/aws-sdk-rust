// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_trunk_interface_input_input(input: &crate::input::AssociateTrunkInterfaceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AssociateTrunkInterface", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("BranchInterfaceId");
    if let Some(var_2) = &input.branch_interface_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TrunkInterfaceId");
    if let Some(var_4) = &input.trunk_interface_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("VlanId");
    if let Some(var_6) = &input.vlan_id {
        scope_5.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("GreKey");
    if let Some(var_8) = &input.gre_key {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ClientToken");
    if let Some(var_10) = &input.client_token {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("DryRun");
    if let Some(var_12) = &input.dry_run {
        scope_11.boolean(*var_12);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

