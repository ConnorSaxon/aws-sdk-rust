// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_media_for_fragment_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMediaForFragmentListInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fragments {
        let mut array_2 = object.key("Fragments").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.stream_arn {
        object.key("StreamARN").string(var_4.as_str());
    }
    if let Some(var_5) = &input.stream_name {
        object.key("StreamName").string(var_5.as_str());
    }
    Ok(())
}

