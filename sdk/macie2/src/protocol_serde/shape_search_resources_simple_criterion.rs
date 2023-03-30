// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search_resources_simple_criterion(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SearchResourcesSimpleCriterion) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.comparator {
        object.key("comparator").string(var_1.as_str());
    }
    if let Some(var_2) = &input.key {
        object.key("key").string(var_2.as_str());
    }
    if let Some(var_3) = &input.values {
        let mut array_4 = object.key("values").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

