// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_package_aggregation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PackageAggregation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.package_names {
        let mut array_2 = object.key("packageNames").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_string_filter::ser_string_filter(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.sort_order {
        object.key("sortOrder").string(var_5.as_str());
    }
    if let Some(var_6) = &input.sort_by {
        object.key("sortBy").string(var_6.as_str());
    }
    Ok(())
}

