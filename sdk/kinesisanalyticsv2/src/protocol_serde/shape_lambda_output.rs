// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_output(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LambdaOutput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceARN").string(var_1.as_str());
    }
    Ok(())
}

