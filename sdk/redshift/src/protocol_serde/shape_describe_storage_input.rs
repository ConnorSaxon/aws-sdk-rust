// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_storage_input_input(input: &crate::input::DescribeStorageInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeStorage", "2012-12-01");
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

