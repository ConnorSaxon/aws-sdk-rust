// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_release_labels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListReleaseLabelsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.filters {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Filters").start_object();
        crate::protocol_serde::shape_release_label_filter::ser_release_label_filter(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

