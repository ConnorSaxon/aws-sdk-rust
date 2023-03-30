// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_pending_log_delivery_configuration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::PendingLogDeliveryConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::PendingLogDeliveryConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LogType") /* LogType com.amazonaws.elasticache#PendingLogDeliveryConfiguration$LogType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::LogType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::LogType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_log_type(var_1);
            }
            ,
            s if s.matches("DestinationType") /* DestinationType com.amazonaws.elasticache#PendingLogDeliveryConfiguration$DestinationType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::DestinationType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::DestinationType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_type(var_2);
            }
            ,
            s if s.matches("DestinationDetails") /* DestinationDetails com.amazonaws.elasticache#PendingLogDeliveryConfiguration$DestinationDetails */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_destination_details::de_destination_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination_details(var_3);
            }
            ,
            s if s.matches("LogFormat") /* LogFormat com.amazonaws.elasticache#PendingLogDeliveryConfiguration$LogFormat */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::LogFormat, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::LogFormat::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_log_format(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

