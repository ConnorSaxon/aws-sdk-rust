// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ebs_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EbsConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ebs_block_device_configs {
        let mut array_2 = object.key("EbsBlockDeviceConfigs").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_ebs_block_device_config::ser_ebs_block_device_config(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.ebs_optimized {
        object.key("EbsOptimized").boolean(*var_5);
    }
    Ok(())
}

