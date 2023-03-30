// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_domain_entry_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDomainEntryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("domainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_entry {
        #[allow(unused_mut)]
        let mut object_3 = object.key("domainEntry").start_object();
        crate::protocol_serde::shape_domain_entry::ser_domain_entry(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

