// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_group_certificate_authorities_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListGroupCertificateAuthoritiesOutput, crate::error::ListGroupCertificateAuthoritiesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListGroupCertificateAuthoritiesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListGroupCertificateAuthoritiesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::ListGroupCertificateAuthoritiesError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListGroupCertificateAuthoritiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::error::ListGroupCertificateAuthoritiesError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListGroupCertificateAuthoritiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListGroupCertificateAuthoritiesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_group_certificate_authorities_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListGroupCertificateAuthoritiesOutput, crate::error::ListGroupCertificateAuthoritiesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_group_certificate_authorities_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_group_certificate_authorities::de_list_group_certificate_authorities(response.body().as_ref(), output).map_err(crate::error::ListGroupCertificateAuthoritiesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_group_certificate_authorities(value: &[u8], mut builder: crate::output::list_group_certificate_authorities_output::Builder) -> Result<crate::output::list_group_certificate_authorities_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "GroupCertificateAuthorities" => {
                        builder = builder.set_group_certificate_authorities(
                            crate::protocol_serde::shape___list_of_group_certificate_authority_properties::de___list_of_group_certificate_authority_properties(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

