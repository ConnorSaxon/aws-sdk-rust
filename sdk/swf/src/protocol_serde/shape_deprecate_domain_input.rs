// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deprecate_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeprecateDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    Ok(())
}

