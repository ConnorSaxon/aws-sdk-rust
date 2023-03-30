// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_device_position_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetDevicePositionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.device_ids {
        let mut array_2 = object.key("DeviceIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

