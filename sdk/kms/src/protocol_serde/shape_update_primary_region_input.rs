// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_primary_region_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePrimaryRegionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.primary_region {
        object.key("PrimaryRegion").string(var_2.as_str());
    }
    Ok(())
}

