// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_file_system_lustre_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DeleteFileSystemLustreConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.skip_final_backup {
        object.key("SkipFinalBackup").boolean(*var_1);
    }
    if let Some(var_2) = &input.final_backup_tags {
        let mut array_3 = object.key("FinalBackupTags").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

