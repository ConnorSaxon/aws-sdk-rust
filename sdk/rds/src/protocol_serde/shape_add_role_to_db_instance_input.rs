// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_role_to_db_instance_input_input(input: &crate::input::AddRoleToDbInstanceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AddRoleToDBInstance", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBInstanceIdentifier");
    if let Some(var_2) = &input.db_instance_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RoleArn");
    if let Some(var_4) = &input.role_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("FeatureName");
    if let Some(var_6) = &input.feature_name {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

