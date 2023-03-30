// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transact_get_items_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TransactGetItemsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transact_items {
        let mut array_2 = object.key("TransactItems").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_transact_get_item::ser_transact_get_item(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_5.as_str());
    }
    Ok(())
}

