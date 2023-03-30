// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_delete_message_batch_result_entry(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DeleteMessageBatchResultEntry, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DeleteMessageBatchResultEntry::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.sqs#DeleteMessageBatchResultEntry$Id */ =>  {
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
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

