// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_sync_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSyncJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sync_role {
        object.key("syncRole").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        #[allow(unused_mut)]
        let mut object_3 = object.key("tags").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    Ok(())
}

