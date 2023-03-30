// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_account_attribute_value(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::AccountAttributeValue, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::AccountAttributeValue::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("attributeValue") /* AttributeValue com.amazonaws.ec2#AccountAttributeValue$AttributeValue */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_attribute_value(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

