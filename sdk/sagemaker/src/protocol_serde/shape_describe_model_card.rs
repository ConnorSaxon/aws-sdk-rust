// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_model_card_input(input: &crate::input::DescribeModelCardInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_model_card_input::ser_describe_model_card_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_model_card_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeModelCardOutput, crate::error::DescribeModelCardError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeModelCardError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeModelCardError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceNotFound" => crate::error::DescribeModelCardError::ResourceNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found::de_resource_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeModelCardError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeModelCardError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_model_card_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeModelCardOutput, crate::error::DescribeModelCardError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_model_card_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_model_card::de_describe_model_card(response.body().as_ref(), output).map_err(crate::error::DescribeModelCardError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_model_card(value: &[u8], mut builder: crate::output::describe_model_card_output::Builder) -> Result<crate::output::describe_model_card_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ModelCardArn" => {
                        builder = builder.set_model_card_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelCardName" => {
                        builder = builder.set_model_card_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelCardVersion" => {
                        builder = builder.set_model_card_version(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "Content" => {
                        builder = builder.set_content(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ModelCardStatus" => {
                        builder = builder.set_model_card_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ModelCardStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "SecurityConfig" => {
                        builder = builder.set_security_config(
                            crate::protocol_serde::shape_model_card_security_config::de_model_card_security_config(tokens)?
                        );
                    }
                    "CreationTime" => {
                        builder = builder.set_creation_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "CreatedBy" => {
                        builder = builder.set_created_by(
                            crate::protocol_serde::shape_user_context::de_user_context(tokens)?
                        );
                    }
                    "LastModifiedTime" => {
                        builder = builder.set_last_modified_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "LastModifiedBy" => {
                        builder = builder.set_last_modified_by(
                            crate::protocol_serde::shape_user_context::de_user_context(tokens)?
                        );
                    }
                    "ModelCardProcessingStatus" => {
                        builder = builder.set_model_card_processing_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ModelCardProcessingStatus::from(u.as_ref())
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

