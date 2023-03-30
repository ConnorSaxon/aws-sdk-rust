// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notifications_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NotificationsConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lambda_function_arn {
        object.key("LambdaFunctionArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sqs_queue_arn {
        object.key("SqsQueueArn").string(var_3.as_str());
    }
    Ok(())
}

