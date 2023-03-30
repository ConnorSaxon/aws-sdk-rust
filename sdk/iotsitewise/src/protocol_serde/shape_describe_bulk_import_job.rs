// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_bulk_import_job_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeBulkImportJobOutput, crate::error::DescribeBulkImportJobError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeBulkImportJobError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => crate::error::DescribeBulkImportJobError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::DescribeBulkImportJobError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DescribeBulkImportJobError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::DescribeBulkImportJobError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeBulkImportJobError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_bulk_import_job_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeBulkImportJobOutput, crate::error::DescribeBulkImportJobError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_bulk_import_job_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_bulk_import_job::de_describe_bulk_import_job(response.body().as_ref(), output).map_err(crate::error::DescribeBulkImportJobError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_bulk_import_job(value: &[u8], mut builder: crate::output::describe_bulk_import_job_output::Builder) -> Result<crate::output::describe_bulk_import_job_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "errorReportLocation" => {
                        builder = builder.set_error_report_location(
                            crate::protocol_serde::shape_error_report_location::de_error_report_location(tokens)?
                        );
                    }
                    "files" => {
                        builder = builder.set_files(
                            crate::protocol_serde::shape_files::de_files(tokens)?
                        );
                    }
                    "jobConfiguration" => {
                        builder = builder.set_job_configuration(
                            crate::protocol_serde::shape_job_configuration::de_job_configuration(tokens)?
                        );
                    }
                    "jobCreationDate" => {
                        builder = builder.set_job_creation_date(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "jobId" => {
                        builder = builder.set_job_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "jobLastUpdateDate" => {
                        builder = builder.set_job_last_update_date(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "jobName" => {
                        builder = builder.set_job_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "jobRoleArn" => {
                        builder = builder.set_job_role_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "jobStatus" => {
                        builder = builder.set_job_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::JobStatus::from(u.as_ref())
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

