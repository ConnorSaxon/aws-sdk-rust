// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFilterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.filter_arn {
        object.key("filterArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.filter_criteria {
        #[allow(unused_mut)]
        let mut object_5 = object.key("filterCriteria").start_object();
        crate::protocol_serde::shape_filter_criteria::ser_filter_criteria(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.reason {
        object.key("reason").string(var_7.as_str());
    }
    Ok(())
}

