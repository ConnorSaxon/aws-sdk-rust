// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_permission_policy_input(input: &crate::input::PutPermissionPolicyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_permission_policy_input::ser_put_permission_policy_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_permission_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutPermissionPolicyOutput, crate::error::PutPermissionPolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutPermissionPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutPermissionPolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "WAFInternalErrorException" => crate::error::PutPermissionPolicyError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutPermissionPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidParameterException" => crate::error::PutPermissionPolicyError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutPermissionPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidPermissionPolicyException" => crate::error::PutPermissionPolicyError::WafInvalidPermissionPolicyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_permission_policy_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_permission_policy_exception::de_waf_invalid_permission_policy_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutPermissionPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFNonexistentItemException" => crate::error::PutPermissionPolicyError::WafNonexistentItemException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_nonexistent_item_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_nonexistent_item_exception::de_waf_nonexistent_item_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutPermissionPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PutPermissionPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_permission_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutPermissionPolicyOutput, crate::error::PutPermissionPolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_permission_policy_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

