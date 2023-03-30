// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_ssl_policy(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SslPolicy, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SslPolicy::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SslProtocols") /* SslProtocols com.amazonaws.elasticloadbalancingv2#SslPolicy$SslProtocols */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ssl_protocols::de_ssl_protocols(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ssl_protocols(var_1);
            }
            ,
            s if s.matches("Ciphers") /* Ciphers com.amazonaws.elasticloadbalancingv2#SslPolicy$Ciphers */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_ciphers::de_ciphers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ciphers(var_2);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.elasticloadbalancingv2#SslPolicy$Name */ =>  {
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
            s if s.matches("SupportedLoadBalancerTypes") /* SupportedLoadBalancerTypes com.amazonaws.elasticloadbalancingv2#SslPolicy$SupportedLoadBalancerTypes */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_list_of_string::de_list_of_string(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_supported_load_balancer_types(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

