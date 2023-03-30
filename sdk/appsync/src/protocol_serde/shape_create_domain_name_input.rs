// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_domain_name_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDomainNameInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("certificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.domain_name {
        object.key("domainName").string(var_3.as_str());
    }
    Ok(())
}

