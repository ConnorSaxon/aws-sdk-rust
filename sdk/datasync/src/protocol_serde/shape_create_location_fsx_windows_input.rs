// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_location_fsx_windows_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLocationFsxWindowsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.subdirectory {
        object.key("Subdirectory").string(var_1.as_str());
    }
    if let Some(var_2) = &input.fsx_filesystem_arn {
        object.key("FsxFilesystemArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.security_group_arns {
        let mut array_4 = object.key("SecurityGroupArns").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag_list_entry::ser_tag_list_entry(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.user {
        object.key("User").string(var_10.as_str());
    }
    if let Some(var_11) = &input.domain {
        object.key("Domain").string(var_11.as_str());
    }
    if let Some(var_12) = &input.password {
        object.key("Password").string(var_12.as_str());
    }
    Ok(())
}

