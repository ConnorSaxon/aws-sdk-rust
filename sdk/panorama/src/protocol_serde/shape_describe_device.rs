// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_device_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDeviceOutput, crate::error::DescribeDeviceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeDeviceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeDeviceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeDeviceError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::error::DescribeDeviceError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDeviceError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::DescribeDeviceError::unhandled("Failed to parse RetryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DescribeDeviceError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::DescribeDeviceError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeDeviceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_device_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDeviceOutput, crate::error::DescribeDeviceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_device_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_device::de_describe_device(response.body().as_ref(), output).map_err(crate::error::DescribeDeviceError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_device(value: &[u8], mut builder: crate::output::describe_device_output::Builder) -> Result<crate::output::describe_device_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "AlternateSoftwares" => {
                        builder = builder.set_alternate_softwares(
                            crate::protocol_serde::shape_alternate_softwares::de_alternate_softwares(tokens)?
                        );
                    }
                    "Arn" => {
                        builder = builder.set_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Brand" => {
                        builder = builder.set_brand(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DeviceBrand::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "CreatedTime" => {
                        builder = builder.set_created_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "CurrentNetworkingStatus" => {
                        builder = builder.set_current_networking_status(
                            crate::protocol_serde::shape_network_status::de_network_status(tokens)?
                        );
                    }
                    "CurrentSoftware" => {
                        builder = builder.set_current_software(
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
                    "DeviceAggregatedStatus" => {
                        builder = builder.set_device_aggregated_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DeviceAggregatedStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "DeviceConnectionStatus" => {
                        builder = builder.set_device_connection_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DeviceConnectionStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "DeviceId" => {
                        builder = builder.set_device_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LatestAlternateSoftware" => {
                        builder = builder.set_latest_alternate_software(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LatestDeviceJob" => {
                        builder = builder.set_latest_device_job(
                            crate::protocol_serde::shape_latest_device_job::de_latest_device_job(tokens)?
                        );
                    }
                    "LatestSoftware" => {
                        builder = builder.set_latest_software(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "LeaseExpirationTime" => {
                        builder = builder.set_lease_expiration_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "Name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "NetworkingConfiguration" => {
                        builder = builder.set_networking_configuration(
                            crate::protocol_serde::shape_network_payload::de_network_payload(tokens)?
                        );
                    }
                    "ProvisioningStatus" => {
                        builder = builder.set_provisioning_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DeviceStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "SerialNumber" => {
                        builder = builder.set_serial_number(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?
                        );
                    }
                    "Type" => {
                        builder = builder.set_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DeviceType::from(u.as_ref())
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

