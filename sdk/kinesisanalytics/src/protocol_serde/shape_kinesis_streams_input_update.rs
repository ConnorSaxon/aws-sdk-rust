// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_kinesis_streams_input_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::KinesisStreamsInputUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_2.as_str());
    }
    Ok(())
}

