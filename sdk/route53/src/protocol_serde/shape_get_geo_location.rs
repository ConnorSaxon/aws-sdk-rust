// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_geo_location_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetGeoLocationOutput, crate::error::GetGeoLocationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetGeoLocationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetGeoLocationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::error::GetGeoLocationError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::error::GetGeoLocationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchGeoLocation" => crate::error::GetGeoLocationError::NoSuchGeoLocation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_geo_location::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_geo_location::de_no_such_geo_location_xml_err(response.body().as_ref(), output).map_err(crate::error::GetGeoLocationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetGeoLocationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_geo_location_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetGeoLocationOutput, crate::error::GetGeoLocationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_geo_location_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_geo_location::de_get_geo_location(response.body().as_ref(), output).map_err(crate::error::GetGeoLocationError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_geo_location(inp: &[u8], mut builder: crate::output::get_geo_location_output::Builder) -> Result<crate::output::get_geo_location_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetGeoLocationResponse") {
                            return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetGeoLocationResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("GeoLocationDetails") /* GeoLocationDetails com.amazonaws.route53.synthetic#GetGeoLocationOutput$GeoLocationDetails */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_geo_location_details::de_geo_location_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_geo_location_details(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

