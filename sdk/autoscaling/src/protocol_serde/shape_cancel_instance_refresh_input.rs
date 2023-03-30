// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_instance_refresh_input_input(input: &crate::input::CancelInstanceRefreshInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CancelInstanceRefresh", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

