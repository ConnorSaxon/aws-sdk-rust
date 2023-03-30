// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_s3_resources_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateS3ResourcesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.member_account_id {
        object.key("memberAccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_resources_update {
        let mut array_3 = object.key("s3ResourcesUpdate").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_s3_resource_classification_update::ser_s3_resource_classification_update(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

