// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_type_version_summary(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TypeVersionSummary, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TypeVersionSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Type") /* Type com.amazonaws.cloudformation#TypeVersionSummary$Type */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::RegistryType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::RegistryType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_1);
            }
            ,
            s if s.matches("TypeName") /* TypeName com.amazonaws.cloudformation#TypeVersionSummary$TypeName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_name(var_2);
            }
            ,
            s if s.matches("VersionId") /* VersionId com.amazonaws.cloudformation#TypeVersionSummary$VersionId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_version_id(var_3);
            }
            ,
            s if s.matches("IsDefaultVersion") /* IsDefaultVersion com.amazonaws.cloudformation#TypeVersionSummary$IsDefaultVersion */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#IsDefaultVersion`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_default_version(var_4);
            }
            ,
            s if s.matches("Arn") /* Arn com.amazonaws.cloudformation#TypeVersionSummary$Arn */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_5);
            }
            ,
            s if s.matches("TimeCreated") /* TimeCreated com.amazonaws.cloudformation#TypeVersionSummary$TimeCreated */ =>  {
                let var_6 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_time_created(var_6);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.cloudformation#TypeVersionSummary$Description */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_7);
            }
            ,
            s if s.matches("PublicVersionNumber") /* PublicVersionNumber com.amazonaws.cloudformation#TypeVersionSummary$PublicVersionNumber */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_version_number(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

