// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_context_keys_for_principal_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetContextKeysForPrincipalPolicyOutput, crate::error::GetContextKeysForPrincipalPolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetContextKeysForPrincipalPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetContextKeysForPrincipalPolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::error::GetContextKeysForPrincipalPolicyError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GetContextKeysForPrincipalPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::GetContextKeysForPrincipalPolicyError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::GetContextKeysForPrincipalPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetContextKeysForPrincipalPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_context_keys_for_principal_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetContextKeysForPrincipalPolicyOutput, crate::error::GetContextKeysForPrincipalPolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_context_keys_for_principal_policy_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_context_keys_for_principal_policy::de_get_context_keys_for_principal_policy(response.body().as_ref(), output).map_err(crate::error::GetContextKeysForPrincipalPolicyError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_context_keys_for_principal_policy(inp: &[u8], mut builder: crate::output::get_context_keys_for_principal_policy_output::Builder) -> Result<crate::output::get_context_keys_for_principal_policy_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GetContextKeysForPrincipalPolicyResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GetContextKeysForPrincipalPolicyResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("GetContextKeysForPrincipalPolicyResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected GetContextKeysForPrincipalPolicyResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("ContextKeyNames") /* ContextKeyNames com.amazonaws.iam.synthetic#GetContextKeysForPrincipalPolicyOutput$ContextKeyNames */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_context_key_names_result_list_type::de_context_key_names_result_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_context_key_names(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected GetContextKeysForPrincipalPolicyResult tag"))
                    };
    Ok(builder)
}

