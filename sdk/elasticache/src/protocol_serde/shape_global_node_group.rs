// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_global_node_group(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::GlobalNodeGroup, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::GlobalNodeGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("GlobalNodeGroupId") /* GlobalNodeGroupId com.amazonaws.elasticache#GlobalNodeGroup$GlobalNodeGroupId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_global_node_group_id(var_1);
            }
            ,
            s if s.matches("Slots") /* Slots com.amazonaws.elasticache#GlobalNodeGroup$Slots */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_slots(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

