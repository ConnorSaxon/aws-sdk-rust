// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_public_dns_namespace_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsNamespaceProperties) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.dns_properties {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DnsProperties").start_object();
        crate::protocol_serde::shape_public_dns_properties_mutable::ser_public_dns_properties_mutable(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

