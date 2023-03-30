// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_merge_profiles_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::MergeProfilesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.field_source_profile_ids {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FieldSourceProfileIds").start_object();
        crate::protocol_serde::shape_field_source_profile_ids::ser_field_source_profile_ids(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.main_profile_id {
        object.key("MainProfileId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.profile_ids_to_be_merged {
        let mut array_5 = object.key("ProfileIdsToBeMerged").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}

