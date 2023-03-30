// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_findings_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFindingsFilterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.finding_criteria {
        #[allow(unused_mut)]
        let mut object_5 = object.key("findingCriteria").start_object();
        crate::protocol_serde::shape_finding_criteria::ser_finding_criteria(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if input.position != 0 {
        object.key("position").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.position).into()));
    }
    Ok(())
}

