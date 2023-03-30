// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_nested_filters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NestedFilters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.nested_property_name {
        object.key("NestedPropertyName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        let mut array_3 = object.key("Filters").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_filter::ser_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

