// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_pipeline_state_input(input: &crate::input::GetPipelineStateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_pipeline_state_input::ser_get_pipeline_state_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_pipeline_state_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetPipelineStateOutput, crate::error::GetPipelineStateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetPipelineStateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetPipelineStateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "PipelineNotFoundException" => crate::error::GetPipelineStateError::PipelineNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pipeline_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pipeline_not_found_exception::de_pipeline_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetPipelineStateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::GetPipelineStateError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetPipelineStateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetPipelineStateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_pipeline_state_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetPipelineStateOutput, crate::error::GetPipelineStateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_pipeline_state_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_pipeline_state::de_get_pipeline_state(response.body().as_ref(), output).map_err(crate::error::GetPipelineStateError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_pipeline_state(value: &[u8], mut builder: crate::output::get_pipeline_state_output::Builder) -> Result<crate::output::get_pipeline_state_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "pipelineName" => {
                        builder = builder.set_pipeline_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "pipelineVersion" => {
                        builder = builder.set_pipeline_version(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "stageStates" => {
                        builder = builder.set_stage_states(
                            crate::protocol_serde::shape_stage_state_list::de_stage_state_list(tokens)?
                        );
                    }
                    "created" => {
                        builder = builder.set_created(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "updated" => {
                        builder = builder.set_updated(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
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

