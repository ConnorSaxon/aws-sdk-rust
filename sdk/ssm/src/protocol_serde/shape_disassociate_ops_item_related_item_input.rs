// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_ops_item_related_item_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateOpsItemRelatedItemInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ops_item_id {
        object.key("OpsItemId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.association_id {
        object.key("AssociationId").string(var_2.as_str());
    }
    Ok(())
}

