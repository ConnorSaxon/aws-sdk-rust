// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_db_proxy_input_input(input: &crate::input::DeleteDbProxyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteDBProxy", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBProxyName");
    if let Some(var_2) = &input.db_proxy_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

