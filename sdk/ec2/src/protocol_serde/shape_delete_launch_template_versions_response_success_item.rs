// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_delete_launch_template_versions_response_success_item(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DeleteLaunchTemplateVersionsResponseSuccessItem, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DeleteLaunchTemplateVersionsResponseSuccessItem::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateId") /* LaunchTemplateId com.amazonaws.ec2#DeleteLaunchTemplateVersionsResponseSuccessItem$LaunchTemplateId */ =>  {
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
            s if s.matches("launchTemplateName") /* LaunchTemplateName com.amazonaws.ec2#DeleteLaunchTemplateVersionsResponseSuccessItem$LaunchTemplateName */ =>  {
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
            s if s.matches("versionNumber") /* VersionNumber com.amazonaws.ec2#DeleteLaunchTemplateVersionsResponseSuccessItem$VersionNumber */ =>  {
                let var_3 =
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
                builder = builder.set_version_number(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

