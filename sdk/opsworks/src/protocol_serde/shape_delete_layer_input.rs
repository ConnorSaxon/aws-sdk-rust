// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_layer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteLayerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.layer_id {
        object.key("LayerId").string(var_1.as_str());
    }
    Ok(())
}

