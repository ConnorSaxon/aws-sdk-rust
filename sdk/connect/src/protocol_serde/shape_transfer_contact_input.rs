// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transfer_contact_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TransferContactInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.contact_flow_id {
        object.key("ContactFlowId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.contact_id {
        object.key("ContactId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.instance_id {
        object.key("InstanceId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.queue_id {
        object.key("QueueId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.user_id {
        object.key("UserId").string(var_6.as_str());
    }
    Ok(())
}

