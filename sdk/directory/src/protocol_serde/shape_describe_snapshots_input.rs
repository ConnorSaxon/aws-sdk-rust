// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_snapshots_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSnapshotsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.snapshot_ids {
        let mut array_3 = object.key("SnapshotIds").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.next_token {
        object.key("NextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    Ok(())
}

