// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_aqua_configuration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::AquaConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::AquaConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AquaStatus") /* AquaStatus com.amazonaws.redshift#AquaConfiguration$AquaStatus */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::AquaStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::AquaStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_aqua_status(var_1);
            }
            ,
            s if s.matches("AquaConfigurationStatus") /* AquaConfigurationStatus com.amazonaws.redshift#AquaConfiguration$AquaConfigurationStatus */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::AquaConfigurationStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::AquaConfigurationStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_aqua_configuration_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

