// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resolver_query_log_config_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResolverQueryLogConfigPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resolver_query_log_config_policy {
        object.key("ResolverQueryLogConfigPolicy").string(var_2.as_str());
    }
    Ok(())
}

