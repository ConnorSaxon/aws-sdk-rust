// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_ml_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMlModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ml_model_id {
        object.key("MLModelId").string(var_1.as_str());
    }
    if input.verbose {
        object.key("Verbose").boolean(input.verbose);
    }
    Ok(())
}

