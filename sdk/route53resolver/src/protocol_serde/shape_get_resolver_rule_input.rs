// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_resolver_rule_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetResolverRuleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resolver_rule_id {
        object.key("ResolverRuleId").string(var_1.as_str());
    }
    Ok(())
}

