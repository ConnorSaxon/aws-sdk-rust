// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_region_settings_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeRegionSettingsOutput, crate::error::DescribeRegionSettingsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeRegionSettingsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeRegionSettingsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::DescribeRegionSettingsError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeRegionSettingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeRegionSettingsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_region_settings_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeRegionSettingsOutput, crate::error::DescribeRegionSettingsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_region_settings_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_region_settings::de_describe_region_settings(response.body().as_ref(), output).map_err(crate::error::DescribeRegionSettingsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_region_settings(value: &[u8], mut builder: crate::output::describe_region_settings_output::Builder) -> Result<crate::output::describe_region_settings_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ResourceTypeManagementPreference" => {
                        builder = builder.set_resource_type_management_preference(
                            crate::protocol_serde::shape_resource_type_management_preference::de_resource_type_management_preference(tokens)?
                        );
                    }
                    "ResourceTypeOptInPreference" => {
                        builder = builder.set_resource_type_opt_in_preference(
                            crate::protocol_serde::shape_resource_type_opt_in_preference::de_resource_type_opt_in_preference(tokens)?
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

