// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_multipart_upload_headers(
                    input: &crate::input::CreateMultipartUploadInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.acl {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("acl", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-acl", header_value);
                        }
    }
    if let Some(inner_3) = &input.cache_control {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("cache_control", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Cache-Control", header_value);
                        }
    }
    if let Some(inner_5) = &input.content_disposition {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_disposition", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Disposition", header_value);
                        }
    }
    if let Some(inner_7) = &input.content_encoding {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_encoding", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Encoding", header_value);
                        }
    }
    if let Some(inner_9) = &input.content_language {
        let formatted_10 = inner_9.as_str();
                        if !formatted_10.is_empty() {
                            let header_value = formatted_10;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_language", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Language", header_value);
                        }
    }
    if let Some(inner_11) = &input.content_type {
        let formatted_12 = inner_11.as_str();
                        if !formatted_12.is_empty() {
                            let header_value = formatted_12;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_type", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Type", header_value);
                        }
    }
    if let Some(inner_13) = &input.expires {
        let formatted_14 = inner_13.fmt(aws_smithy_types::date_time::Format::HttpDate)?;
                        if !formatted_14.is_empty() {
                            let header_value = formatted_14;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("expires", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Expires", header_value);
                        }
    }
    if let Some(inner_15) = &input.grant_full_control {
        let formatted_16 = inner_15.as_str();
                        if !formatted_16.is_empty() {
                            let header_value = formatted_16;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("grant_full_control", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-grant-full-control", header_value);
                        }
    }
    if let Some(inner_17) = &input.grant_read {
        let formatted_18 = inner_17.as_str();
                        if !formatted_18.is_empty() {
                            let header_value = formatted_18;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("grant_read", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-grant-read", header_value);
                        }
    }
    if let Some(inner_19) = &input.grant_read_acp {
        let formatted_20 = inner_19.as_str();
                        if !formatted_20.is_empty() {
                            let header_value = formatted_20;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("grant_read_acp", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-grant-read-acp", header_value);
                        }
    }
    if let Some(inner_21) = &input.grant_write_acp {
        let formatted_22 = inner_21.as_str();
                        if !formatted_22.is_empty() {
                            let header_value = formatted_22;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("grant_write_acp", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-grant-write-acp", header_value);
                        }
    }
    if let Some(inner_23) = &input.server_side_encryption {
        let formatted_24 = inner_23.as_str();
                        if !formatted_24.is_empty() {
                            let header_value = formatted_24;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("server_side_encryption", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption", header_value);
                        }
    }
    if let Some(inner_25) = &input.storage_class {
        let formatted_26 = inner_25.as_str();
                        if !formatted_26.is_empty() {
                            let header_value = formatted_26;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("storage_class", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-storage-class", header_value);
                        }
    }
    if let Some(inner_27) = &input.website_redirect_location {
        let formatted_28 = inner_27.as_str();
                        if !formatted_28.is_empty() {
                            let header_value = formatted_28;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("website_redirect_location", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-website-redirect-location", header_value);
                        }
    }
    if let Some(inner_29) = &input.sse_customer_algorithm {
        let formatted_30 = inner_29.as_str();
                        if !formatted_30.is_empty() {
                            let header_value = formatted_30;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
                        }
    }
    if let Some(inner_31) = &input.sse_customer_key {
        let formatted_32 = inner_31.as_str();
                        if !formatted_32.is_empty() {
                            let header_value = formatted_32;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_key", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
                        }
    }
    if let Some(inner_33) = &input.sse_customer_key_md5 {
        let formatted_34 = inner_33.as_str();
                        if !formatted_34.is_empty() {
                            let header_value = formatted_34;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_key_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
                        }
    }
    if let Some(inner_35) = &input.ssekms_key_id {
        let formatted_36 = inner_35.as_str();
                        if !formatted_36.is_empty() {
                            let header_value = formatted_36;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("ssekms_key_id", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-aws-kms-key-id", header_value);
                        }
    }
    if let Some(inner_37) = &input.ssekms_encryption_context {
        let formatted_38 = inner_37.as_str();
                        if !formatted_38.is_empty() {
                            let header_value = formatted_38;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("ssekms_encryption_context", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-context", header_value);
                        }
    }
    if input.bucket_key_enabled {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(input.bucket_key_enabled);
        let formatted_39 = encoder.encode();
                        if !formatted_39.is_empty() {
                            let header_value = formatted_39;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("bucket_key_enabled", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-bucket-key-enabled", header_value);
                        }
    }
    if let Some(inner_40) = &input.request_payer {
        let formatted_41 = inner_40.as_str();
                        if !formatted_41.is_empty() {
                            let header_value = formatted_41;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("request_payer", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-request-payer", header_value);
                        }
    }
    if let Some(inner_42) = &input.tagging {
        let formatted_43 = inner_42.as_str();
                        if !formatted_43.is_empty() {
                            let header_value = formatted_43;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("tagging", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-tagging", header_value);
                        }
    }
    if let Some(inner_44) = &input.object_lock_mode {
        let formatted_45 = inner_44.as_str();
                        if !formatted_45.is_empty() {
                            let header_value = formatted_45;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("object_lock_mode", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-object-lock-mode", header_value);
                        }
    }
    if let Some(inner_46) = &input.object_lock_retain_until_date {
        let formatted_47 = inner_46.fmt(aws_smithy_types::date_time::Format::DateTime)?;
                        if !formatted_47.is_empty() {
                            let header_value = formatted_47;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("object_lock_retain_until_date", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-object-lock-retain-until-date", header_value);
                        }
    }
    if let Some(inner_48) = &input.object_lock_legal_hold_status {
        let formatted_49 = inner_48.as_str();
                        if !formatted_49.is_empty() {
                            let header_value = formatted_49;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("object_lock_legal_hold_status", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-object-lock-legal-hold", header_value);
                        }
    }
    if let Some(inner_50) = &input.expected_bucket_owner {
        let formatted_51 = inner_50.as_str();
                        if !formatted_51.is_empty() {
                            let header_value = formatted_51;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("expected_bucket_owner", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-expected-bucket-owner", header_value);
                        }
    }
    if let Some(inner_52) = &input.checksum_algorithm {
        let formatted_53 = inner_52.as_str();
                        if !formatted_53.is_empty() {
                            let header_value = formatted_53;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-checksum-algorithm", header_value);
                        }
    }
    if let Some(inner_54) = &input.metadata {
         {
            for (k, v) in inner_54 {
                                use std::str::FromStr;
                                let header_name = http::header::HeaderName::from_str(&format!("{}{}", "x-amz-meta-", &k)).map_err(|err| {
                                    aws_smithy_http::operation::error::BuildError::invalid_field("metadata", format!("`{k}` cannot be used as a header name: {err}"))
                                })?;
                                let header_value = v.as_str();
                                let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                    aws_smithy_http::operation::error::BuildError::invalid_field("metadata", format!(
                                        "`{}` cannot be used as a header value: {}",
                                        v,
                                        err
                                    ))
                                })?;
                                builder = builder.header(header_name, header_value);
                            }
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_multipart_upload_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateMultipartUploadOutput, crate::error::CreateMultipartUploadError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateMultipartUploadError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::CreateMultipartUploadError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_multipart_upload_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateMultipartUploadOutput, crate::error::CreateMultipartUploadError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_multipart_upload_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_multipart_upload::de_create_multipart_upload(response.body().as_ref(), output).map_err(crate::error::CreateMultipartUploadError::unhandled)?;
        output = output.set_abort_date(
            crate::protocol_serde::shape_create_multipart_upload_output::de_abort_date_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse AbortDate from header `x-amz-abort-date"))?
        );
        output = output.set_abort_rule_id(
            crate::protocol_serde::shape_create_multipart_upload_output::de_abort_rule_id_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse AbortRuleId from header `x-amz-abort-rule-id"))?
        );
        output = output.set_bucket_key_enabled(
            crate::protocol_serde::shape_create_multipart_upload_output::de_bucket_key_enabled_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse BucketKeyEnabled from header `x-amz-server-side-encryption-bucket-key-enabled"))?
        );
        output = output.set_checksum_algorithm(
            crate::protocol_serde::shape_create_multipart_upload_output::de_checksum_algorithm_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse ChecksumAlgorithm from header `x-amz-checksum-algorithm"))?
        );
        output = output.set_request_charged(
            crate::protocol_serde::shape_create_multipart_upload_output::de_request_charged_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged"))?
        );
        output = output.set_sse_customer_algorithm(
            crate::protocol_serde::shape_create_multipart_upload_output::de_sse_customer_algorithm_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse SSECustomerAlgorithm from header `x-amz-server-side-encryption-customer-algorithm"))?
        );
        output = output.set_sse_customer_key_md5(
            crate::protocol_serde::shape_create_multipart_upload_output::de_sse_customer_key_md5_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse SSECustomerKeyMD5 from header `x-amz-server-side-encryption-customer-key-MD5"))?
        );
        output = output.set_ssekms_encryption_context(
            crate::protocol_serde::shape_create_multipart_upload_output::de_ssekms_encryption_context_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse SSEKMSEncryptionContext from header `x-amz-server-side-encryption-context"))?
        );
        output = output.set_ssekms_key_id(
            crate::protocol_serde::shape_create_multipart_upload_output::de_ssekms_key_id_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse SSEKMSKeyId from header `x-amz-server-side-encryption-aws-kms-key-id"))?
        );
        output = output.set_server_side_encryption(
            crate::protocol_serde::shape_create_multipart_upload_output::de_server_side_encryption_header(response.headers())
                                    .map_err(|_|crate::error::CreateMultipartUploadError::unhandled("Failed to parse ServerSideEncryption from header `x-amz-server-side-encryption"))?
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string));
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_multipart_upload(inp: &[u8], mut builder: crate::output::create_multipart_upload_output::Builder) -> Result<crate::output::create_multipart_upload_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("InitiateMultipartUploadResult") {
                            return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected InitiateMultipartUploadResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("UploadId") /* UploadId com.amazonaws.s3.synthetic#CreateMultipartUploadOutput$UploadId */ =>  {
                let var_55 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upload_id(var_55);
            }
            ,
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3.synthetic#CreateMultipartUploadOutput$Bucket */ =>  {
                let var_56 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_56);
            }
            ,
            s if s.matches("Key") /* Key com.amazonaws.s3.synthetic#CreateMultipartUploadOutput$Key */ =>  {
                let var_57 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_57);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

