// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_project_input(input: &crate::input::DescribeProjectInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_project_input::ser_describe_project_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_project_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeProjectOutput, crate::error::DescribeProjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeProjectError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeProjectError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_project_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeProjectOutput, crate::error::DescribeProjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_project_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_project::de_describe_project(response.body().as_ref(), output).map_err(crate::error::DescribeProjectError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_project(value: &[u8], mut builder: crate::output::describe_project_output::Builder) -> Result<crate::output::describe_project_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ProjectArn" => {
                        builder = builder.set_project_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ProjectName" => {
                        builder = builder.set_project_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ProjectId" => {
                        builder = builder.set_project_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ProjectDescription" => {
                        builder = builder.set_project_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ServiceCatalogProvisioningDetails" => {
                        builder = builder.set_service_catalog_provisioning_details(
                            crate::protocol_serde::shape_service_catalog_provisioning_details::de_service_catalog_provisioning_details(tokens)?
                        );
                    }
                    "ServiceCatalogProvisionedProductDetails" => {
                        builder = builder.set_service_catalog_provisioned_product_details(
                            crate::protocol_serde::shape_service_catalog_provisioned_product_details::de_service_catalog_provisioned_product_details(tokens)?
                        );
                    }
                    "ProjectStatus" => {
                        builder = builder.set_project_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ProjectStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "CreatedBy" => {
                        builder = builder.set_created_by(
                            crate::protocol_serde::shape_user_context::de_user_context(tokens)?
                        );
                    }
                    "CreationTime" => {
                        builder = builder.set_creation_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
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

