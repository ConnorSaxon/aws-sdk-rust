// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_list_outgoing_typed_links(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BatchListOutgoingTypedLinks) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.object_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ObjectReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.filter_attribute_ranges {
        let mut array_4 = object.key("FilterAttributeRanges").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_typed_link_attribute_range::ser_typed_link_attribute_range(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.filter_typed_link {
        #[allow(unused_mut)]
        let mut object_8 = object.key("FilterTypedLink").start_object();
        crate::protocol_serde::shape_typed_link_schema_and_facet_name::ser_typed_link_schema_and_facet_name(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.next_token {
        object.key("NextToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    Ok(())
}

