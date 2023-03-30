// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_kg_key_pair_ids(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::KgKeyPairIds, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::KgKeyPairIds::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("KeyGroupId") /* KeyGroupId com.amazonaws.cloudfront#KGKeyPairIds$KeyGroupId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key_group_id(var_1);
            }
            ,
            s if s.matches("KeyPairIds") /* KeyPairIds com.amazonaws.cloudfront#KGKeyPairIds$KeyPairIds */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_key_pair_ids::de_key_pair_ids(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_key_pair_ids(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

