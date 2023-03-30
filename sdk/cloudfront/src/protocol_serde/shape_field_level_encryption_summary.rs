// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_field_level_encryption_summary(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::FieldLevelEncryptionSummary, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::FieldLevelEncryptionSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudfront#FieldLevelEncryptionSummary$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("LastModifiedTime") /* LastModifiedTime com.amazonaws.cloudfront#FieldLevelEncryptionSummary$LastModifiedTime */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudfront#timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified_time(var_2);
            }
            ,
            s if s.matches("Comment") /* Comment com.amazonaws.cloudfront#FieldLevelEncryptionSummary$Comment */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_comment(var_3);
            }
            ,
            s if s.matches("QueryArgProfileConfig") /* QueryArgProfileConfig com.amazonaws.cloudfront#FieldLevelEncryptionSummary$QueryArgProfileConfig */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_query_arg_profile_config::de_query_arg_profile_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_query_arg_profile_config(var_4);
            }
            ,
            s if s.matches("ContentTypeProfileConfig") /* ContentTypeProfileConfig com.amazonaws.cloudfront#FieldLevelEncryptionSummary$ContentTypeProfileConfig */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_content_type_profile_config::de_content_type_profile_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_content_type_profile_config(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

