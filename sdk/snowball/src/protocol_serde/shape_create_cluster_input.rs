// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cluster_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateClusterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_type {
        object.key("JobType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resources {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Resources").start_object();
        crate::protocol_serde::shape_job_resource::ser_job_resource(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.on_device_service_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("OnDeviceServiceConfiguration").start_object();
        crate::protocol_serde::shape_on_device_service_configuration::ser_on_device_service_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.address_id {
        object.key("AddressId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.kms_key_arn {
        object.key("KmsKeyARN").string(var_8.as_str());
    }
    if let Some(var_9) = &input.role_arn {
        object.key("RoleARN").string(var_9.as_str());
    }
    if let Some(var_10) = &input.snowball_type {
        object.key("SnowballType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.shipping_option {
        object.key("ShippingOption").string(var_11.as_str());
    }
    if let Some(var_12) = &input.notification {
        #[allow(unused_mut)]
        let mut object_13 = object.key("Notification").start_object();
        crate::protocol_serde::shape_notification::ser_notification(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.forwarding_address_id {
        object.key("ForwardingAddressId").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tax_documents {
        #[allow(unused_mut)]
        let mut object_16 = object.key("TaxDocuments").start_object();
        crate::protocol_serde::shape_tax_documents::ser_tax_documents(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.remote_management {
        object.key("RemoteManagement").string(var_17.as_str());
    }
    Ok(())
}

