// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connector_profile_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConnectorProfileConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connector_profile_properties {
        #[allow(unused_mut)]
        let mut object_2 = object.key("connectorProfileProperties").start_object();
        crate::protocol_serde::shape_connector_profile_properties::ser_connector_profile_properties(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.connector_profile_credentials {
        #[allow(unused_mut)]
        let mut object_4 = object.key("connectorProfileCredentials").start_object();
        crate::protocol_serde::shape_connector_profile_credentials::ser_connector_profile_credentials(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

