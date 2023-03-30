// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_cancel_capacity_reservation_fleet_error(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::CancelCapacityReservationFleetError, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::CancelCapacityReservationFleetError::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("code") /* Code com.amazonaws.ec2#CancelCapacityReservationFleetError$Code */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_code(var_1);
            }
            ,
            s if s.matches("message") /* Message com.amazonaws.ec2#CancelCapacityReservationFleetError$Message */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

