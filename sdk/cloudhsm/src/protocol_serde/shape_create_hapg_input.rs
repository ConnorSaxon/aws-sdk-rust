// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_hapg_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateHapgInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.label {
        object.key("Label").string(var_1.as_str());
    }
    Ok(())
}

