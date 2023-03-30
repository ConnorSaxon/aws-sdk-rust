// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_change_info(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ChangeInfo, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ChangeInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.route53#ChangeInfo$Id */ =>  {
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
            s if s.matches("Status") /* Status com.amazonaws.route53#ChangeInfo$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::ChangeStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ChangeStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            s if s.matches("SubmittedAt") /* SubmittedAt com.amazonaws.route53#ChangeInfo$SubmittedAt */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.route53#TimeStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_submitted_at(var_3);
            }
            ,
            s if s.matches("Comment") /* Comment com.amazonaws.route53#ChangeInfo$Comment */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_comment(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

