// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_instance_block_device_mapping(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::InstanceBlockDeviceMapping, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::InstanceBlockDeviceMapping::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("deviceName") /* DeviceName com.amazonaws.ec2#InstanceBlockDeviceMapping$DeviceName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_device_name(var_1);
            }
            ,
            s if s.matches("ebs") /* Ebs com.amazonaws.ec2#InstanceBlockDeviceMapping$Ebs */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_ebs_instance_block_device::de_ebs_instance_block_device(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ebs(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

