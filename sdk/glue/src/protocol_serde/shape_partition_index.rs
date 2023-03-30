// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_partition_index(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PartitionIndex) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.keys {
        let mut array_2 = object.key("Keys").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.index_name {
        object.key("IndexName").string(var_4.as_str());
    }
    Ok(())
}

