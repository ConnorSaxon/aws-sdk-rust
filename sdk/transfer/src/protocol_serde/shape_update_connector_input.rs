// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_connector_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateConnectorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connector_id {
        object.key("ConnectorId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.url {
        object.key("Url").string(var_2.as_str());
    }
    if let Some(var_3) = &input.as2_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("As2Config").start_object();
        crate::protocol_serde::shape_as2_connector_config::ser_as2_connector_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.access_role {
        object.key("AccessRole").string(var_5.as_str());
    }
    if let Some(var_6) = &input.logging_role {
        object.key("LoggingRole").string(var_6.as_str());
    }
    Ok(())
}

