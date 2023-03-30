// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_forwarded_values(input: &crate::model::ForwardedValues, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.query_string {
        let mut inner_writer = scope.start_el("QueryString").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_1).encode()
        );
    }
    if let Some(var_2) = &input.cookies {
        let inner_writer = scope.start_el("Cookies");
        crate::protocol_serde::shape_cookie_preference::ser_cookie_preference(var_2, inner_writer)?
    }
    if let Some(var_3) = &input.headers {
        let inner_writer = scope.start_el("Headers");
        crate::protocol_serde::shape_headers::ser_headers(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.query_string_cache_keys {
        let inner_writer = scope.start_el("QueryStringCacheKeys");
        crate::protocol_serde::shape_query_string_cache_keys::ser_query_string_cache_keys(var_4, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_forwarded_values(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ForwardedValues, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ForwardedValues::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("QueryString") /* QueryString com.amazonaws.cloudfront#ForwardedValues$QueryString */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudfront#boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_query_string(var_5);
            }
            ,
            s if s.matches("Cookies") /* Cookies com.amazonaws.cloudfront#ForwardedValues$Cookies */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_cookie_preference::de_cookie_preference(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cookies(var_6);
            }
            ,
            s if s.matches("Headers") /* Headers com.amazonaws.cloudfront#ForwardedValues$Headers */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_headers::de_headers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_headers(var_7);
            }
            ,
            s if s.matches("QueryStringCacheKeys") /* QueryStringCacheKeys com.amazonaws.cloudfront#ForwardedValues$QueryStringCacheKeys */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_query_string_cache_keys::de_query_string_cache_keys(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_query_string_cache_keys(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

