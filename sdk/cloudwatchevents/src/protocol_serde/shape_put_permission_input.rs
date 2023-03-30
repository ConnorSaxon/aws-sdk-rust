// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_permission_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPermissionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_bus_name {
        object.key("EventBusName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.action {
        object.key("Action").string(var_2.as_str());
    }
    if let Some(var_3) = &input.principal {
        object.key("Principal").string(var_3.as_str());
    }
    if let Some(var_4) = &input.statement_id {
        object.key("StatementId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.condition {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Condition").start_object();
        crate::protocol_serde::shape_condition::ser_condition(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.policy {
        object.key("Policy").string(var_7.as_str());
    }
    Ok(())
}

