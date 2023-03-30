// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_origin_access_control_summary(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::OriginAccessControlSummary, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::OriginAccessControlSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudfront#OriginAccessControlSummary$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.cloudfront#OriginAccessControlSummary$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.cloudfront#OriginAccessControlSummary$Name */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_3);
            }
            ,
            s if s.matches("SigningProtocol") /* SigningProtocol com.amazonaws.cloudfront#OriginAccessControlSummary$SigningProtocol */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::OriginAccessControlSigningProtocols, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::OriginAccessControlSigningProtocols::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_signing_protocol(var_4);
            }
            ,
            s if s.matches("SigningBehavior") /* SigningBehavior com.amazonaws.cloudfront#OriginAccessControlSummary$SigningBehavior */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::OriginAccessControlSigningBehaviors, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::OriginAccessControlSigningBehaviors::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_signing_behavior(var_5);
            }
            ,
            s if s.matches("OriginAccessControlOriginType") /* OriginAccessControlOriginType com.amazonaws.cloudfront#OriginAccessControlSummary$OriginAccessControlOriginType */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::model::OriginAccessControlOriginTypes, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::OriginAccessControlOriginTypes::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_origin_access_control_origin_type(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

