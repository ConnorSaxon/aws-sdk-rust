// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_backup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CopyBackupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_backup_id {
        object.key("SourceBackupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source_region {
        object.key("SourceRegion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.copy_tags {
        object.key("CopyTags").boolean(*var_5);
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

