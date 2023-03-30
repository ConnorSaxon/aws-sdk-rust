// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_data_quality_rule_recommendation_run_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartDataQualityRuleRecommendationRunInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data_source {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DataSource").start_object();
        crate::protocol_serde::shape_data_source::ser_data_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.role {
        object.key("Role").string(var_3.as_str());
    }
    if let Some(var_4) = &input.number_of_workers {
        object.key("NumberOfWorkers").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.timeout {
        object.key("Timeout").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.created_ruleset_name {
        object.key("CreatedRulesetName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.client_token {
        object.key("ClientToken").string(var_7.as_str());
    }
    Ok(())
}

