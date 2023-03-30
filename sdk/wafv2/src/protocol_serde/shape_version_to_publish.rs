// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_version_to_publish(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VersionToPublish) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.associated_rule_group_arn {
        object.key("AssociatedRuleGroupArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.forecasted_lifetime {
        object.key("ForecastedLifetime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

