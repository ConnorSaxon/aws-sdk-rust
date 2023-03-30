// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_topic_input_input(input: &crate::input::DeleteTopicInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteTopic", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TopicArn");
    if let Some(var_2) = &input.topic_arn {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

