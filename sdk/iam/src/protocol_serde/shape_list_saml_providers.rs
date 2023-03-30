// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_saml_providers_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSamlProvidersOutput, crate::error::ListSAMLProvidersError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListSAMLProvidersError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListSAMLProvidersError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceFailure" => crate::error::ListSAMLProvidersError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ListSAMLProvidersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListSAMLProvidersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_saml_providers_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSamlProvidersOutput, crate::error::ListSAMLProvidersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_saml_providers_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_saml_providers::de_list_saml_providers(response.body().as_ref(), output).map_err(crate::error::ListSAMLProvidersError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_saml_providers(inp: &[u8], mut builder: crate::output::list_saml_providers_output::Builder) -> Result<crate::output::list_saml_providers_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ListSAMLProvidersResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ListSAMLProvidersResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ListSAMLProvidersResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ListSAMLProvidersResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("SAMLProviderList") /* SAMLProviderList com.amazonaws.iam.synthetic#ListSAMLProvidersOutput$SAMLProviderList */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_saml_provider_list_type::de_saml_provider_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_saml_provider_list(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ListSAMLProvidersResult tag"))
                    };
    Ok(builder)
}

