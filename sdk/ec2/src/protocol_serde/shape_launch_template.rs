// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_launch_template(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LaunchTemplate, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LaunchTemplate::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateId") /* LaunchTemplateId com.amazonaws.ec2#LaunchTemplate$LaunchTemplateId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_launch_template_id(var_1);
            }
            ,
            s if s.matches("launchTemplateName") /* LaunchTemplateName com.amazonaws.ec2#LaunchTemplate$LaunchTemplateName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_launch_template_name(var_2);
            }
            ,
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#LaunchTemplate$CreateTime */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_3);
            }
            ,
            s if s.matches("createdBy") /* CreatedBy com.amazonaws.ec2#LaunchTemplate$CreatedBy */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_created_by(var_4);
            }
            ,
            s if s.matches("defaultVersionNumber") /* DefaultVersionNumber com.amazonaws.ec2#LaunchTemplate$DefaultVersionNumber */ =>  {
                let var_5 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_version_number(var_5);
            }
            ,
            s if s.matches("latestVersionNumber") /* LatestVersionNumber com.amazonaws.ec2#LaunchTemplate$LatestVersionNumber */ =>  {
                let var_6 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_latest_version_number(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#LaunchTemplate$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

