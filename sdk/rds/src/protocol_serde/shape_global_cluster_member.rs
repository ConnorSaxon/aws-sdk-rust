// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_global_cluster_member(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::GlobalClusterMember, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::GlobalClusterMember::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBClusterArn") /* DBClusterArn com.amazonaws.rds#GlobalClusterMember$DBClusterArn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_arn(var_1);
            }
            ,
            s if s.matches("Readers") /* Readers com.amazonaws.rds#GlobalClusterMember$Readers */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_readers_arn_list::de_readers_arn_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_readers(var_2);
            }
            ,
            s if s.matches("IsWriter") /* IsWriter com.amazonaws.rds#GlobalClusterMember$IsWriter */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_writer(var_3);
            }
            ,
            s if s.matches("GlobalWriteForwardingStatus") /* GlobalWriteForwardingStatus com.amazonaws.rds#GlobalClusterMember$GlobalWriteForwardingStatus */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::WriteForwardingStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::WriteForwardingStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_global_write_forwarding_status(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

