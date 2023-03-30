// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_function_code_signing_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutFunctionCodeSigningConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.code_signing_config_arn {
        object.key("CodeSigningConfigArn").string(var_1.as_str());
    }
    Ok(())
}

