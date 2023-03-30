// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_network_interface_count(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::NetworkInterfaceCount) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Min");
    if let Some(var_2) = &input.min {
        scope_1.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Max");
    if let Some(var_4) = &input.max {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

pub fn de_network_interface_count(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::NetworkInterfaceCount, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::NetworkInterfaceCount::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("min") /* Min com.amazonaws.ec2#NetworkInterfaceCount$Min */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min(var_5);
            }
            ,
            s if s.matches("max") /* Max com.amazonaws.ec2#NetworkInterfaceCount$Max */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

