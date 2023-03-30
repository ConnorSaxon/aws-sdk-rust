// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_persistent_storage_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PersistentStorageConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.size_in_gi_b {
        object.key("sizeInGiB").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    Ok(())
}

