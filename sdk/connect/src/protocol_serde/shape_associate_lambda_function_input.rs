// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_lambda_function_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateLambdaFunctionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.function_arn {
        object.key("FunctionArn").string(var_1.as_str());
    }
    Ok(())
}

