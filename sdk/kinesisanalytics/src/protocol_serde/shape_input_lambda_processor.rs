// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input_lambda_processor(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InputLambdaProcessor) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("RoleARN").string(var_2.as_str());
    }
    Ok(())
}

