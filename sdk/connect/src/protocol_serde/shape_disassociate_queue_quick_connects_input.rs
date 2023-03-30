// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_queue_quick_connects_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateQueueQuickConnectsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.quick_connect_ids {
        let mut array_2 = object.key("QuickConnectIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

