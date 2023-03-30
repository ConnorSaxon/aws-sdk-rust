// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_type_configuration_details(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TypeConfigurationDetails, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TypeConfigurationDetails::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Arn") /* Arn com.amazonaws.cloudformation#TypeConfigurationDetails$Arn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_1);
            }
            ,
            s if s.matches("Alias") /* Alias com.amazonaws.cloudformation#TypeConfigurationDetails$Alias */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_alias(var_2);
            }
            ,
            s if s.matches("Configuration") /* Configuration com.amazonaws.cloudformation#TypeConfigurationDetails$Configuration */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_configuration(var_3);
            }
            ,
            s if s.matches("LastUpdated") /* LastUpdated com.amazonaws.cloudformation#TypeConfigurationDetails$LastUpdated */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_updated(var_4);
            }
            ,
            s if s.matches("TypeArn") /* TypeArn com.amazonaws.cloudformation#TypeConfigurationDetails$TypeArn */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_arn(var_5);
            }
            ,
            s if s.matches("TypeName") /* TypeName com.amazonaws.cloudformation#TypeConfigurationDetails$TypeName */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_name(var_6);
            }
            ,
            s if s.matches("IsDefaultConfiguration") /* IsDefaultConfiguration com.amazonaws.cloudformation#TypeConfigurationDetails$IsDefaultConfiguration */ =>  {
                let var_7 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#IsDefaultConfiguration`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_default_configuration(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

