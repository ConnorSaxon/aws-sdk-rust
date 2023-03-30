// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_refresh_schemas_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RefreshSchemasInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.endpoint_arn {
        object.key("EndpointArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.replication_instance_arn {
        object.key("ReplicationInstanceArn").string(var_2.as_str());
    }
    Ok(())
}

