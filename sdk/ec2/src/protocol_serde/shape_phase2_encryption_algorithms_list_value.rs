// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_phase2_encryption_algorithms_list_value(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Phase2EncryptionAlgorithmsListValue, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Phase2EncryptionAlgorithmsListValue::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("value") /* Value com.amazonaws.ec2#Phase2EncryptionAlgorithmsListValue$Value */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

