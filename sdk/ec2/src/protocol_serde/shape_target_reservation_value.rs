// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_target_reservation_value(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TargetReservationValue, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TargetReservationValue::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("reservationValue") /* ReservationValue com.amazonaws.ec2#TargetReservationValue$ReservationValue */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_reservation_value::de_reservation_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_reservation_value(var_1);
            }
            ,
            s if s.matches("targetConfiguration") /* TargetConfiguration com.amazonaws.ec2#TargetReservationValue$TargetConfiguration */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_target_configuration::de_target_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_configuration(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

