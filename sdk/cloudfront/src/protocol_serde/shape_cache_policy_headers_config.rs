// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cache_policy_headers_config(input: &crate::model::CachePolicyHeadersConfig, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.header_behavior {
        let mut inner_writer = scope.start_el("HeaderBehavior").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    if let Some(var_2) = &input.headers {
        let inner_writer = scope.start_el("Headers");
        crate::protocol_serde::shape_headers::ser_headers(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_cache_policy_headers_config(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::CachePolicyHeadersConfig, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::CachePolicyHeadersConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("HeaderBehavior") /* HeaderBehavior com.amazonaws.cloudfront#CachePolicyHeadersConfig$HeaderBehavior */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::model::CachePolicyHeaderBehavior, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::CachePolicyHeaderBehavior::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_header_behavior(var_3);
            }
            ,
            s if s.matches("Headers") /* Headers com.amazonaws.cloudfront#CachePolicyHeadersConfig$Headers */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_headers::de_headers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_headers(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

