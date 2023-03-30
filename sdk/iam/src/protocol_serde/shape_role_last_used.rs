// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_role_last_used(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::RoleLastUsed, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::RoleLastUsed::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LastUsedDate") /* LastUsedDate com.amazonaws.iam#RoleLastUsed$LastUsedDate */ =>  {
                let var_1 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_last_used_date(var_1);
            }
            ,
            s if s.matches("Region") /* Region com.amazonaws.iam#RoleLastUsed$Region */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

