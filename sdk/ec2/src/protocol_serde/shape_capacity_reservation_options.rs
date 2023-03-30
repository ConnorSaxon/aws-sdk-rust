// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_capacity_reservation_options(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::CapacityReservationOptions, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::CapacityReservationOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("usageStrategy") /* UsageStrategy com.amazonaws.ec2#CapacityReservationOptions$UsageStrategy */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::FleetCapacityReservationUsageStrategy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::FleetCapacityReservationUsageStrategy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_usage_strategy(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

