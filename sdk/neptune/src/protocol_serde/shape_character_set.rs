// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_character_set(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::CharacterSet, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::CharacterSet::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CharacterSetName") /* CharacterSetName com.amazonaws.neptune#CharacterSet$CharacterSetName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_character_set_name(var_1);
            }
            ,
            s if s.matches("CharacterSetDescription") /* CharacterSetDescription com.amazonaws.neptune#CharacterSet$CharacterSetDescription */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_character_set_description(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

