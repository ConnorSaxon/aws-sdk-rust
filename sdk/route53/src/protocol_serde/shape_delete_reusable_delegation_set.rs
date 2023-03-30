// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_reusable_delegation_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteReusableDelegationSetOutput, crate::error::DeleteReusableDelegationSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteReusableDelegationSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteReusableDelegationSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DelegationSetInUse" => crate::error::DeleteReusableDelegationSetError::DelegationSetInUse({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::delegation_set_in_use::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_delegation_set_in_use::de_delegation_set_in_use_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteReusableDelegationSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DelegationSetNotReusable" => crate::error::DeleteReusableDelegationSetError::DelegationSetNotReusable({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::delegation_set_not_reusable::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_delegation_set_not_reusable::de_delegation_set_not_reusable_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteReusableDelegationSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::error::DeleteReusableDelegationSetError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteReusableDelegationSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchDelegationSet" => crate::error::DeleteReusableDelegationSetError::NoSuchDelegationSet({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_delegation_set::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_delegation_set::de_no_such_delegation_set_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteReusableDelegationSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteReusableDelegationSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_reusable_delegation_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteReusableDelegationSetOutput, crate::error::DeleteReusableDelegationSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_reusable_delegation_set_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

