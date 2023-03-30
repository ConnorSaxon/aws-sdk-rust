// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_fleet_advisor_collector_input(input: &crate::input::CreateFleetAdvisorCollectorInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_fleet_advisor_collector_input::ser_create_fleet_advisor_collector_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_fleet_advisor_collector_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateFleetAdvisorCollectorOutput, crate::error::CreateFleetAdvisorCollectorError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateFleetAdvisorCollectorError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedFault" => crate::error::CreateFleetAdvisorCollectorError::AccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_fault::de_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateFault" => crate::error::CreateFleetAdvisorCollectorError::InvalidResourceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_fault::de_invalid_resource_state_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceQuotaExceededFault" => crate::error::CreateFleetAdvisorCollectorError::ResourceQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_quota_exceeded_fault::de_resource_quota_exceeded_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "S3AccessDeniedFault" => crate::error::CreateFleetAdvisorCollectorError::S3AccessDeniedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::s3_access_denied_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_s3_access_denied_fault::de_s3_access_denied_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "S3ResourceNotFoundFault" => crate::error::CreateFleetAdvisorCollectorError::S3ResourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::s3_resource_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_s3_resource_not_found_fault::de_s3_resource_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateFleetAdvisorCollectorError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_fleet_advisor_collector_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateFleetAdvisorCollectorOutput, crate::error::CreateFleetAdvisorCollectorError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_fleet_advisor_collector_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_fleet_advisor_collector::de_create_fleet_advisor_collector(response.body().as_ref(), output).map_err(crate::error::CreateFleetAdvisorCollectorError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_fleet_advisor_collector(value: &[u8], mut builder: crate::output::create_fleet_advisor_collector_output::Builder) -> Result<crate::output::create_fleet_advisor_collector_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "CollectorReferencedId" => {
                        builder = builder.set_collector_referenced_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "CollectorName" => {
                        builder = builder.set_collector_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ServiceAccessRoleArn" => {
                        builder = builder.set_service_access_role_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "S3BucketName" => {
                        builder = builder.set_s3_bucket_name(
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

