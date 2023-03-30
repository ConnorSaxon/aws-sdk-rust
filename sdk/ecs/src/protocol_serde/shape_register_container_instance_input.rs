// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_container_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterContainerInstanceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_identity_document {
        object.key("instanceIdentityDocument").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_identity_document_signature {
        object.key("instanceIdentityDocumentSignature").string(var_3.as_str());
    }
    if let Some(var_4) = &input.total_resources {
        let mut array_5 = object.key("totalResources").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_resource::ser_resource(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.version_info {
        #[allow(unused_mut)]
        let mut object_9 = object.key("versionInfo").start_object();
        crate::protocol_serde::shape_version_info::ser_version_info(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.container_instance_arn {
        object.key("containerInstanceArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.attributes {
        let mut array_12 = object.key("attributes").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_attribute::ser_attribute(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.platform_devices {
        let mut array_16 = object.key("platformDevices").start_array();
        for item_17 in var_15 {
             {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_platform_device::ser_platform_device(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("tags").start_array();
        for item_21 in var_19 {
             {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

