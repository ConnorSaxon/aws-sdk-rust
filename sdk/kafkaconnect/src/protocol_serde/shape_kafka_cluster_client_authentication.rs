// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_kafka_cluster_client_authentication(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::KafkaClusterClientAuthentication) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.authentication_type {
        object.key("authenticationType").string(var_1.as_str());
    }
    Ok(())
}

