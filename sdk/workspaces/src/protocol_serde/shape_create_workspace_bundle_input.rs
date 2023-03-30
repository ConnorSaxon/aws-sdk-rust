// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_workspace_bundle_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkspaceBundleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bundle_name {
        object.key("BundleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.bundle_description {
        object.key("BundleDescription").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_id {
        object.key("ImageId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.compute_type {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ComputeType").start_object();
        crate::protocol_serde::shape_compute_type::ser_compute_type(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.user_storage {
        #[allow(unused_mut)]
        let mut object_7 = object.key("UserStorage").start_object();
        crate::protocol_serde::shape_user_storage::ser_user_storage(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.root_storage {
        #[allow(unused_mut)]
        let mut object_9 = object.key("RootStorage").start_object();
        crate::protocol_serde::shape_root_storage::ser_root_storage(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("Tags").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}

