// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_redirect_action_config(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::RedirectActionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Protocol");
    if let Some(var_2) = &input.protocol {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Port");
    if let Some(var_4) = &input.port {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Host");
    if let Some(var_6) = &input.host {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Path");
    if let Some(var_8) = &input.path {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Query");
    if let Some(var_10) = &input.query {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("StatusCode");
    if let Some(var_12) = &input.status_code {
        scope_11.string(var_12.as_str());
    }
    Ok(())
}

pub fn de_redirect_action_config(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::RedirectActionConfig, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::RedirectActionConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Protocol") /* Protocol com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$Protocol */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_protocol(var_13);
            }
            ,
            s if s.matches("Port") /* Port com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$Port */ =>  {
                let var_14 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_port(var_14);
            }
            ,
            s if s.matches("Host") /* Host com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$Host */ =>  {
                let var_15 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_host(var_15);
            }
            ,
            s if s.matches("Path") /* Path com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$Path */ =>  {
                let var_16 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_path(var_16);
            }
            ,
            s if s.matches("Query") /* Query com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$Query */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_query(var_17);
            }
            ,
            s if s.matches("StatusCode") /* StatusCode com.amazonaws.elasticloadbalancingv2#RedirectActionConfig$StatusCode */ =>  {
                let var_18 =
                    Some(
                        Result::<crate::model::RedirectActionStatusCodeEnum, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::RedirectActionStatusCodeEnum::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status_code(var_18);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

