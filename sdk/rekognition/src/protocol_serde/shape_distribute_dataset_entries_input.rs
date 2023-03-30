// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_distribute_dataset_entries_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DistributeDatasetEntriesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.datasets {
        let mut array_2 = object.key("Datasets").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_distribute_dataset::ser_distribute_dataset(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

