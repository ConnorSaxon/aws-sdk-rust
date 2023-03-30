// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_fragment_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FragmentSelector) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.timestamp_range {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TimestampRange").start_object();
        crate::protocol_serde::shape_timestamp_range::ser_timestamp_range(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

