// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_network_interface_permission_input_input(input: &crate::input::DeleteNetworkInterfacePermissionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteNetworkInterfacePermission", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NetworkInterfacePermissionId");
    if let Some(var_2) = &input.network_interface_permission_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Force");
    if let Some(var_4) = &input.force {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

