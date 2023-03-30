// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_session_stickiness_config(input: &crate::model::SessionStickinessConfig, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.idle_ttl {
        let mut inner_writer = scope.start_el("IdleTTL").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_1).encode()
        );
    }
    if let Some(var_2) = &input.maximum_ttl {
        let mut inner_writer = scope.start_el("MaximumTTL").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_2).encode()
        );
    }
    scope.finish();
    Ok(())
}

pub fn de_session_stickiness_config(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SessionStickinessConfig, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SessionStickinessConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("IdleTTL") /* IdleTTL com.amazonaws.cloudfront#SessionStickinessConfig$IdleTTL */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_idle_ttl(var_3);
            }
            ,
            s if s.matches("MaximumTTL") /* MaximumTTL com.amazonaws.cloudfront#SessionStickinessConfig$MaximumTTL */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_maximum_ttl(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

