// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_conformance_pack_compliance_details_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetConformancePackComplianceDetailsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.conformance_pack_name {
        object.key("ConformancePackName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Filters").start_object();
        crate::protocol_serde::shape_conformance_pack_evaluation_filters::ser_conformance_pack_evaluation_filters(&mut object_3, var_2)?;
        object_3.finish();
    }
    if input.limit != 0 {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.limit).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

