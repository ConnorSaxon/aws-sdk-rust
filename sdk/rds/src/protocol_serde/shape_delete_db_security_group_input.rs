// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_db_security_group_input_input(input: &crate::input::DeleteDbSecurityGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteDBSecurityGroup", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBSecurityGroupName");
    if let Some(var_2) = &input.db_security_group_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

