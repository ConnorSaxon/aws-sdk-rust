// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ebs_instance_block_device_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::EbsInstanceBlockDeviceSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DeleteOnTermination");
    if let Some(var_2) = &input.delete_on_termination {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VolumeId");
    if let Some(var_4) = &input.volume_id {
        scope_3.string(var_4);
    }
    Ok(())
}

