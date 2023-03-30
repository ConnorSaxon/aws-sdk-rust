// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_on_premises_instance_input(input: &crate::input::GetOnPremisesInstanceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_on_premises_instance_input::ser_get_on_premises_instance_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_on_premises_instance_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetOnPremisesInstanceOutput, crate::error::GetOnPremisesInstanceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetOnPremisesInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetOnPremisesInstanceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InstanceNameRequiredException" => crate::error::GetOnPremisesInstanceError::InstanceNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::instance_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_instance_name_required_exception::de_instance_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetOnPremisesInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InstanceNotRegisteredException" => crate::error::GetOnPremisesInstanceError::InstanceNotRegisteredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::instance_not_registered_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_instance_not_registered_exception::de_instance_not_registered_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetOnPremisesInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceNameException" => crate::error::GetOnPremisesInstanceError::InvalidInstanceNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_name_exception::de_invalid_instance_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetOnPremisesInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetOnPremisesInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_on_premises_instance_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetOnPremisesInstanceOutput, crate::error::GetOnPremisesInstanceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_on_premises_instance_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_on_premises_instance::de_get_on_premises_instance(response.body().as_ref(), output).map_err(crate::error::GetOnPremisesInstanceError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_on_premises_instance(value: &[u8], mut builder: crate::output::get_on_premises_instance_output::Builder) -> Result<crate::output::get_on_premises_instance_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "instanceInfo" => {
                        builder = builder.set_instance_info(
                            crate::protocol_serde::shape_instance_info::de_instance_info(tokens)?
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

