// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_ops_item_related_item_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateOpsItemRelatedItemInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ops_item_id {
        object.key("OpsItemId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.association_type {
        object.key("AssociationType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_type {
        object.key("ResourceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.resource_uri {
        object.key("ResourceUri").string(var_4.as_str());
    }
    Ok(())
}

