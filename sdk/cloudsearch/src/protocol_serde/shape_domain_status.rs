// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_domain_status(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DomainStatus, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DomainStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DomainId") /* DomainId com.amazonaws.cloudsearch#DomainStatus$DomainId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_domain_id(var_1);
            }
            ,
            s if s.matches("DomainName") /* DomainName com.amazonaws.cloudsearch#DomainStatus$DomainName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_domain_name(var_2);
            }
            ,
            s if s.matches("ARN") /* ARN com.amazonaws.cloudsearch#DomainStatus$ARN */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_3);
            }
            ,
            s if s.matches("Created") /* Created com.amazonaws.cloudsearch#DomainStatus$Created */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudsearch#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_created(var_4);
            }
            ,
            s if s.matches("Deleted") /* Deleted com.amazonaws.cloudsearch#DomainStatus$Deleted */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudsearch#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_deleted(var_5);
            }
            ,
            s if s.matches("DocService") /* DocService com.amazonaws.cloudsearch#DomainStatus$DocService */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_service_endpoint::de_service_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_doc_service(var_6);
            }
            ,
            s if s.matches("SearchService") /* SearchService com.amazonaws.cloudsearch#DomainStatus$SearchService */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_service_endpoint::de_service_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_search_service(var_7);
            }
            ,
            s if s.matches("RequiresIndexDocuments") /* RequiresIndexDocuments com.amazonaws.cloudsearch#DomainStatus$RequiresIndexDocuments */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudsearch#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_requires_index_documents(var_8);
            }
            ,
            s if s.matches("Processing") /* Processing com.amazonaws.cloudsearch#DomainStatus$Processing */ =>  {
                let var_9 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudsearch#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_processing(var_9);
            }
            ,
            s if s.matches("SearchInstanceType") /* SearchInstanceType com.amazonaws.cloudsearch#DomainStatus$SearchInstanceType */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_search_instance_type(var_10);
            }
            ,
            s if s.matches("SearchPartitionCount") /* SearchPartitionCount com.amazonaws.cloudsearch#DomainStatus$SearchPartitionCount */ =>  {
                let var_11 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudsearch#PartitionCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_search_partition_count(var_11);
            }
            ,
            s if s.matches("SearchInstanceCount") /* SearchInstanceCount com.amazonaws.cloudsearch#DomainStatus$SearchInstanceCount */ =>  {
                let var_12 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudsearch#InstanceCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_search_instance_count(var_12);
            }
            ,
            s if s.matches("Limits") /* Limits com.amazonaws.cloudsearch#DomainStatus$Limits */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_limits::de_limits(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_limits(var_13);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

