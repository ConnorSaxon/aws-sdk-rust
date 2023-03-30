// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_provision_product_input(input: &crate::input::ProvisionProductInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_provision_product_input::ser_provision_product_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_product_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ProvisionProductOutput, crate::error::ProvisionProductError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ProvisionProductError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ProvisionProductError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateResourceException" => crate::error::ProvisionProductError::DuplicateResourceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_resource_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_resource_exception::de_duplicate_resource_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ProvisionProductError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParametersException" => crate::error::ProvisionProductError::InvalidParametersException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameters_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameters_exception::de_invalid_parameters_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ProvisionProductError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::ProvisionProductError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ProvisionProductError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ProvisionProductError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_product_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ProvisionProductOutput, crate::error::ProvisionProductError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::provision_product_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_provision_product::de_provision_product(response.body().as_ref(), output).map_err(crate::error::ProvisionProductError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_provision_product(value: &[u8], mut builder: crate::output::provision_product_output::Builder) -> Result<crate::output::provision_product_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "RecordDetail" => {
                        builder = builder.set_record_detail(
                            crate::protocol_serde::shape_record_detail::de_record_detail(tokens)?
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

