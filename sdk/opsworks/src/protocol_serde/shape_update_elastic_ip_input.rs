// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_elastic_ip_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateElasticIpInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.elastic_ip {
        object.key("ElasticIp").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    Ok(())
}

