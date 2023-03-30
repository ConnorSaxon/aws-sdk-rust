// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transfer_domain_to_another_aws_account_input(input: &crate::input::TransferDomainToAnotherAwsAccountInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_transfer_domain_to_another_aws_account_input::ser_transfer_domain_to_another_aws_account_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_transfer_domain_to_another_aws_account_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TransferDomainToAnotherAwsAccountOutput, crate::error::TransferDomainToAnotherAwsAccountError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateRequest" => crate::error::TransferDomainToAnotherAwsAccountError::DuplicateRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_request::de_duplicate_request_json_err(response.body().as_ref(), output).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::error::TransferDomainToAnotherAwsAccountError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_json_err(response.body().as_ref(), output).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationLimitExceeded" => crate::error::TransferDomainToAnotherAwsAccountError::OperationLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_limit_exceeded::de_operation_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedTLD" => crate::error::TransferDomainToAnotherAwsAccountError::UnsupportedTld({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_tld::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_tld::de_unsupported_tld_json_err(response.body().as_ref(), output).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::TransferDomainToAnotherAwsAccountError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_transfer_domain_to_another_aws_account_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TransferDomainToAnotherAwsAccountOutput, crate::error::TransferDomainToAnotherAwsAccountError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::transfer_domain_to_another_aws_account_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_transfer_domain_to_another_aws_account::de_transfer_domain_to_another_aws_account(response.body().as_ref(), output).map_err(crate::error::TransferDomainToAnotherAwsAccountError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_transfer_domain_to_another_aws_account(value: &[u8], mut builder: crate::output::transfer_domain_to_another_aws_account_output::Builder) -> Result<crate::output::transfer_domain_to_another_aws_account_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "OperationId" => {
                        builder = builder.set_operation_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Password" => {
                        builder = builder.set_password(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
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

