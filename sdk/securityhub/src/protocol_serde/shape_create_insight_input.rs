// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_insight_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateInsightInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filters {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Filters").start_object();
        crate::protocol_serde::shape_aws_security_finding_filters::ser_aws_security_finding_filters(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.group_by_attribute {
        object.key("GroupByAttribute").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    Ok(())
}

