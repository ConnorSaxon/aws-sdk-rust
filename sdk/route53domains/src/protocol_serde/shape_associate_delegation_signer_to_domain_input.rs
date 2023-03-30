// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_delegation_signer_to_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateDelegationSignerToDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.signing_attributes {
        #[allow(unused_mut)]
        let mut object_3 = object.key("SigningAttributes").start_object();
        crate::protocol_serde::shape_dnssec_signing_attributes::ser_dnssec_signing_attributes(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

