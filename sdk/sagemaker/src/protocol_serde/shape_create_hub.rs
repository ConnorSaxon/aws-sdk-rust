// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_hub_input(input: &crate::input::CreateHubInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_hub_input::ser_create_hub_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_hub_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateHubOutput, crate::error::CreateHubError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateHubError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateHubError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceInUse" => crate::error::CreateHubError::ResourceInUse({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_in_use::de_resource_in_use_json_err(response.body().as_ref(), output).map_err(crate::error::CreateHubError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceLimitExceeded" => crate::error::CreateHubError::ResourceLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_limit_exceeded::de_resource_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::CreateHubError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateHubError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_hub_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateHubOutput, crate::error::CreateHubError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_hub_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_hub::de_create_hub(response.body().as_ref(), output).map_err(crate::error::CreateHubError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_hub(value: &[u8], mut builder: crate::output::create_hub_output::Builder) -> Result<crate::output::create_hub_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "HubArn" => {
                        builder = builder.set_hub_arn(
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

