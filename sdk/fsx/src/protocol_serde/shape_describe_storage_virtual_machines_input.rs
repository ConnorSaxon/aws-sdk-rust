// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_storage_virtual_machines_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStorageVirtualMachinesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.storage_virtual_machine_ids {
        let mut array_2 = object.key("StorageVirtualMachineIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.filters {
        let mut array_5 = object.key("Filters").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_storage_virtual_machine_filter::ser_storage_virtual_machine_filter(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.next_token {
        object.key("NextToken").string(var_9.as_str());
    }
    Ok(())
}

