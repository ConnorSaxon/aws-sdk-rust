// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_vpc_link_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetVpcLinkOutput, crate::error::GetVpcLinkError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetVpcLinkError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetVpcLinkError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "NotFoundException" => crate::error::GetVpcLinkError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetVpcLinkError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetVpcLinkError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetVpcLinkError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetVpcLinkError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_vpc_link_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetVpcLinkOutput, crate::error::GetVpcLinkError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_vpc_link_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_vpc_link::de_get_vpc_link(response.body().as_ref(), output).map_err(crate::error::GetVpcLinkError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_vpc_link(value: &[u8], mut builder: crate::output::get_vpc_link_output::Builder) -> Result<crate::output::get_vpc_link_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "createdDate" => {
                        builder = builder.set_created_date(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                        );
                    }
                    "name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "securityGroupIds" => {
                        builder = builder.set_security_group_ids(
                            crate::protocol_serde::shape_security_group_id_list::de_security_group_id_list(tokens)?
                        );
                    }
                    "subnetIds" => {
                        builder = builder.set_subnet_ids(
                            crate::protocol_serde::shape_subnet_id_list::de_subnet_id_list(tokens)?
                        );
                    }
                    "tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_tags::de_tags(tokens)?
                        );
                    }
                    "vpcLinkId" => {
                        builder = builder.set_vpc_link_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "vpcLinkStatus" => {
                        builder = builder.set_vpc_link_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::VpcLinkStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "vpcLinkStatusMessage" => {
                        builder = builder.set_vpc_link_status_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "vpcLinkVersion" => {
                        builder = builder.set_vpc_link_version(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::VpcLinkVersion::from(u.as_ref())
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

