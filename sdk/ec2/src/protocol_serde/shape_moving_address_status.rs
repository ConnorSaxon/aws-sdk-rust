// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_moving_address_status(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::MovingAddressStatus, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::MovingAddressStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("moveStatus") /* MoveStatus com.amazonaws.ec2#MovingAddressStatus$MoveStatus */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::MoveStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::MoveStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_move_status(var_1);
            }
            ,
            s if s.matches("publicIp") /* PublicIp com.amazonaws.ec2#MovingAddressStatus$PublicIp */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_ip(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

