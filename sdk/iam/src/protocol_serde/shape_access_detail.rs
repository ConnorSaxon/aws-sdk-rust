// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_access_detail(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::AccessDetail, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::AccessDetail::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ServiceName") /* ServiceName com.amazonaws.iam#AccessDetail$ServiceName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_name(var_1);
            }
            ,
            s if s.matches("ServiceNamespace") /* ServiceNamespace com.amazonaws.iam#AccessDetail$ServiceNamespace */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_namespace(var_2);
            }
            ,
            s if s.matches("Region") /* Region com.amazonaws.iam#AccessDetail$Region */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_3);
            }
            ,
            s if s.matches("EntityPath") /* EntityPath com.amazonaws.iam#AccessDetail$EntityPath */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_entity_path(var_4);
            }
            ,
            s if s.matches("LastAuthenticatedTime") /* LastAuthenticatedTime com.amazonaws.iam#AccessDetail$LastAuthenticatedTime */ =>  {
                let var_5 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_last_authenticated_time(var_5);
            }
            ,
            s if s.matches("TotalAuthenticatedEntities") /* TotalAuthenticatedEntities com.amazonaws.iam#AccessDetail$TotalAuthenticatedEntities */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.iam#integerType`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_total_authenticated_entities(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

