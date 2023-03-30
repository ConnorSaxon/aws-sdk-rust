// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_volume_status_info(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::VolumeStatusInfo, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::VolumeStatusInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("details") /* Details com.amazonaws.ec2#VolumeStatusInfo$Details */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_volume_status_details_list::de_volume_status_details_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_details(var_1);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#VolumeStatusInfo$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::VolumeStatusInfoStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::VolumeStatusInfoStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

