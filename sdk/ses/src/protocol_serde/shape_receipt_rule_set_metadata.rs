// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_receipt_rule_set_metadata(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ReceiptRuleSetMetadata, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ReceiptRuleSetMetadata::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Name") /* Name com.amazonaws.ses#ReceiptRuleSetMetadata$Name */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_1);
            }
            ,
            s if s.matches("CreatedTimestamp") /* CreatedTimestamp com.amazonaws.ses#ReceiptRuleSetMetadata$CreatedTimestamp */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ses#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_created_timestamp(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

