// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resolver_rule_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResolverRulePolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resolver_rule_policy {
        object.key("ResolverRulePolicy").string(var_2.as_str());
    }
    Ok(())
}

