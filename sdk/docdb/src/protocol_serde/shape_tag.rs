// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_tag(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

pub fn de_tag(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Tag, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Tag::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Key") /* Key com.amazonaws.docdb#Tag$Key */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_5);
            }
            ,
            s if s.matches("Value") /* Value com.amazonaws.docdb#Tag$Value */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

