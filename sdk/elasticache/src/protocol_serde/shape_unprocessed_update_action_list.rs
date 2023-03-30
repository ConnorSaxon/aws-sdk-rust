// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_unprocessed_update_action_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::UnprocessedUpdateAction>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("UnprocessedUpdateAction") /* member com.amazonaws.elasticache#UnprocessedUpdateActionList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_unprocessed_update_action::de_unprocessed_update_action(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

