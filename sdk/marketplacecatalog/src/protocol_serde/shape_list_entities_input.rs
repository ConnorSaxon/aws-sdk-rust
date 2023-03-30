// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_entities_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListEntitiesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog {
        object.key("Catalog").string(var_1.as_str());
    }
    if let Some(var_2) = &input.entity_type {
        object.key("EntityType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.filter_list {
        let mut array_4 = object.key("FilterList").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_filter::ser_filter(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sort {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Sort").start_object();
        crate::protocol_serde::shape_sort::ser_sort(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

