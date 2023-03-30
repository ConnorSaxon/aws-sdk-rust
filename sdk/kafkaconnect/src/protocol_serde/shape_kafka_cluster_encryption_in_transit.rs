// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_kafka_cluster_encryption_in_transit(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::KafkaClusterEncryptionInTransit) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.encryption_type {
        object.key("encryptionType").string(var_1.as_str());
    }
    Ok(())
}

