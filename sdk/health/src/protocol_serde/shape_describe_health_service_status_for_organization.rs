// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_health_service_status_for_organization_input(_input: &crate::input::DescribeHealthServiceStatusForOrganizationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_health_service_status_for_organization_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHealthServiceStatusForOrganizationOutput, crate::error::DescribeHealthServiceStatusForOrganizationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeHealthServiceStatusForOrganizationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeHealthServiceStatusForOrganizationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_health_service_status_for_organization_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHealthServiceStatusForOrganizationOutput, crate::error::DescribeHealthServiceStatusForOrganizationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_health_service_status_for_organization_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_health_service_status_for_organization::de_describe_health_service_status_for_organization(response.body().as_ref(), output).map_err(crate::error::DescribeHealthServiceStatusForOrganizationError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_health_service_status_for_organization(value: &[u8], mut builder: crate::output::describe_health_service_status_for_organization_output::Builder) -> Result<crate::output::describe_health_service_status_for_organization_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "healthServiceAccessStatusForOrganization" => {
                        builder = builder.set_health_service_access_status_for_organization(
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

