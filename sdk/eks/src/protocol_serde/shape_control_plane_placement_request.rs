// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_control_plane_placement_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ControlPlanePlacementRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.group_name {
        object.key("groupName").string(var_1.as_str());
    }
    Ok(())
}

