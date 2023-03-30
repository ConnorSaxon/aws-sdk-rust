// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_required_activated_type(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::RequiredActivatedType, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::RequiredActivatedType::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TypeNameAlias") /* TypeNameAlias com.amazonaws.cloudformation#RequiredActivatedType$TypeNameAlias */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_name_alias(var_1);
            }
            ,
            s if s.matches("OriginalTypeName") /* OriginalTypeName com.amazonaws.cloudformation#RequiredActivatedType$OriginalTypeName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_original_type_name(var_2);
            }
            ,
            s if s.matches("PublisherId") /* PublisherId com.amazonaws.cloudformation#RequiredActivatedType$PublisherId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_publisher_id(var_3);
            }
            ,
            s if s.matches("SupportedMajorVersions") /* SupportedMajorVersions com.amazonaws.cloudformation#RequiredActivatedType$SupportedMajorVersions */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_supported_major_versions::de_supported_major_versions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_supported_major_versions(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

