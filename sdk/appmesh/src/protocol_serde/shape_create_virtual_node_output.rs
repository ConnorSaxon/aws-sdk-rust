// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_virtual_node_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualNodeData>, crate::error::CreateVirtualNodeError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_virtual_node_data::de_virtual_node_data_payload(body).map_err(crate::error::CreateVirtualNodeError::unhandled)
    }).transpose()
}

