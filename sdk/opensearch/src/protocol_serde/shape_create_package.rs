// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_package_input(input: &crate::input::CreatePackageInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_package_input::ser_create_package_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_package_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreatePackageOutput, crate::error::CreatePackageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreatePackageError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreatePackageError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreatePackageError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BaseException" => crate::error::CreatePackageError::BaseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::base_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalException" => crate::error::CreatePackageError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTypeException" => crate::error::CreatePackageError::InvalidTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_type_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_type_exception::de_invalid_type_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::CreatePackageError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceAlreadyExistsException" => crate::error::CreatePackageError::ResourceAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_already_exists_exception::de_resource_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::CreatePackageError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreatePackageError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_package_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreatePackageOutput, crate::error::CreatePackageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_package_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_package::de_create_package(response.body().as_ref(), output).map_err(crate::error::CreatePackageError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_package(value: &[u8], mut builder: crate::output::create_package_output::Builder) -> Result<crate::output::create_package_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "PackageDetails" => {
                        builder = builder.set_package_details(
                            crate::protocol_serde::shape_package_details::de_package_details(tokens)?
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

