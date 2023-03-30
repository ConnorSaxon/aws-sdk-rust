// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_volume_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateVolumeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.volume_type {
        object.key("VolumeType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.ontap_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("OntapConfiguration").start_object();
        crate::protocol_serde::shape_create_ontap_volume_configuration::ser_create_ontap_volume_configuration(&mut object_5, var_4)?;
        object_5.finish();
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
    if let Some(var_10) = &input.open_zfs_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("OpenZFSConfiguration").start_object();
        crate::protocol_serde::shape_create_open_zfs_volume_configuration::ser_create_open_zfs_volume_configuration(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

