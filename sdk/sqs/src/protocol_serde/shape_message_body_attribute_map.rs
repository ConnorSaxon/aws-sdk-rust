// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_message_body_attribute_map_entry(decoder: &mut aws_smithy_xml::decode::ScopedDecoder, out: &mut std::collections::HashMap<std::string::String, crate::model::MessageAttributeValue>) -> Result<(), aws_smithy_xml::decode::XmlDecodeError> {
    let mut k: Option<std::string::String> = None;
    let mut v: Option<crate::model::MessageAttributeValue> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Name") /* key com.amazonaws.sqs#MessageBodyAttributeMap$key */ =>  {
                k = Some(
                    Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                )
            }
            ,
            s if s.matches("Value") /* value com.amazonaws.sqs#MessageBodyAttributeMap$value */ =>  {
                v = Some(
                    crate::protocol_serde::shape_message_attribute_value::de_message_attribute_value(&mut tag)
                    ?
                )
            }
            ,
            _ => {}
        }
    }
    let k = k.ok_or_else(||aws_smithy_xml::decode::XmlDecodeError::custom("missing key map entry"))?;
                        let v = v.ok_or_else(||aws_smithy_xml::decode::XmlDecodeError::custom("missing value map entry"))?;
                        out.insert(k, v);
                        Ok(())
}

