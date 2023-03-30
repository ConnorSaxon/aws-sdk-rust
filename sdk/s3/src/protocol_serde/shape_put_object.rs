// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_object_headers(
                    input: &crate::input::PutObjectInput,
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
    if input.content_length != 0 {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(input.content_length);
        let formatted_11 = encoder.encode();
                        if !formatted_11.is_empty() {
                            let header_value = formatted_11;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_length", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Length", header_value);
                        }
    }
    if let Some(inner_12) = &input.content_md5 {
        let formatted_13 = inner_12.as_str();
                        if !formatted_13.is_empty() {
                            let header_value = formatted_13;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-MD5", header_value);
                        }
    }
    if let Some(inner_14) = &input.content_type {
        let formatted_15 = inner_14.as_str();
                        if !formatted_15.is_empty() {
                            let header_value = formatted_15;
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
    if let Some(inner_16) = &input.checksum_algorithm {
        let formatted_17 = inner_16.as_str();
                        if !formatted_17.is_empty() {
                            let header_value = formatted_17;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-sdk-checksum-algorithm", header_value);
                        }
    }
    if let Some(inner_18) = &input.checksum_crc32 {
        let formatted_19 = inner_18.as_str();
                        if !formatted_19.is_empty() {
                            let header_value = formatted_19;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_crc32", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-checksum-crc32", header_value);
                        }
    }
    if let Some(inner_20) = &input.checksum_crc32_c {
        let formatted_21 = inner_20.as_str();
                        if !formatted_21.is_empty() {
                            let header_value = formatted_21;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_crc32_c", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-checksum-crc32c", header_value);
                        }
    }
    if let Some(inner_22) = &input.checksum_sha1 {
        let formatted_23 = inner_22.as_str();
                        if !formatted_23.is_empty() {
                            let header_value = formatted_23;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_sha1", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-checksum-sha1", header_value);
                        }
    }
    if let Some(inner_24) = &input.checksum_sha256 {
        let formatted_25 = inner_24.as_str();
                        if !formatted_25.is_empty() {
                            let header_value = formatted_25;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("checksum_sha256", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-checksum-sha256", header_value);
                        }
    }
    if let Some(inner_26) = &input.expires {
        let formatted_27 = inner_26.fmt(aws_smithy_types::date_time::Format::HttpDate)?;
                        if !formatted_27.is_empty() {
                            let header_value = formatted_27;
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
    if let Some(inner_28) = &input.grant_full_control {
        let formatted_29 = inner_28.as_str();
                        if !formatted_29.is_empty() {
                            let header_value = formatted_29;
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
    if let Some(inner_30) = &input.grant_read {
        let formatted_31 = inner_30.as_str();
                        if !formatted_31.is_empty() {
                            let header_value = formatted_31;
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
    if let Some(inner_32) = &input.grant_read_acp {
        let formatted_33 = inner_32.as_str();
                        if !formatted_33.is_empty() {
                            let header_value = formatted_33;
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
    if let Some(inner_34) = &input.grant_write_acp {
        let formatted_35 = inner_34.as_str();
                        if !formatted_35.is_empty() {
                            let header_value = formatted_35;
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
    if let Some(inner_36) = &input.server_side_encryption {
        let formatted_37 = inner_36.as_str();
                        if !formatted_37.is_empty() {
                            let header_value = formatted_37;
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
    if let Some(inner_38) = &input.storage_class {
        let formatted_39 = inner_38.as_str();
                        if !formatted_39.is_empty() {
                            let header_value = formatted_39;
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
    if let Some(inner_40) = &input.website_redirect_location {
        let formatted_41 = inner_40.as_str();
                        if !formatted_41.is_empty() {
                            let header_value = formatted_41;
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
    if let Some(inner_42) = &input.sse_customer_algorithm {
        let formatted_43 = inner_42.as_str();
                        if !formatted_43.is_empty() {
                            let header_value = formatted_43;
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
    if let Some(inner_44) = &input.sse_customer_key {
        let formatted_45 = inner_44.as_str();
                        if !formatted_45.is_empty() {
                            let header_value = formatted_45;
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
    if let Some(inner_46) = &input.sse_customer_key_md5 {
        let formatted_47 = inner_46.as_str();
                        if !formatted_47.is_empty() {
                            let header_value = formatted_47;
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
    if let Some(inner_48) = &input.ssekms_key_id {
        let formatted_49 = inner_48.as_str();
                        if !formatted_49.is_empty() {
                            let header_value = formatted_49;
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
    if let Some(inner_50) = &input.ssekms_encryption_context {
        let formatted_51 = inner_50.as_str();
                        if !formatted_51.is_empty() {
                            let header_value = formatted_51;
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
        let formatted_52 = encoder.encode();
                        if !formatted_52.is_empty() {
                            let header_value = formatted_52;
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
    if let Some(inner_53) = &input.request_payer {
        let formatted_54 = inner_53.as_str();
                        if !formatted_54.is_empty() {
                            let header_value = formatted_54;
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
    if let Some(inner_55) = &input.tagging {
        let formatted_56 = inner_55.as_str();
                        if !formatted_56.is_empty() {
                            let header_value = formatted_56;
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
    if let Some(inner_57) = &input.object_lock_mode {
        let formatted_58 = inner_57.as_str();
                        if !formatted_58.is_empty() {
                            let header_value = formatted_58;
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
    if let Some(inner_59) = &input.object_lock_retain_until_date {
        let formatted_60 = inner_59.fmt(aws_smithy_types::date_time::Format::DateTime)?;
                        if !formatted_60.is_empty() {
                            let header_value = formatted_60;
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
    if let Some(inner_61) = &input.object_lock_legal_hold_status {
        let formatted_62 = inner_61.as_str();
                        if !formatted_62.is_empty() {
                            let header_value = formatted_62;
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
    if let Some(inner_63) = &input.expected_bucket_owner {
        let formatted_64 = inner_63.as_str();
                        if !formatted_64.is_empty() {
                            let header_value = formatted_64;
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
    if let Some(inner_65) = &input.metadata {
         {
            for (k, v) in inner_65 {
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
pub fn de_put_object_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutObjectError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::PutObjectError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_object_output::Builder::default();
        let _ = response;
        output = output.set_bucket_key_enabled(
            crate::protocol_serde::shape_put_object_output::de_bucket_key_enabled_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse BucketKeyEnabled from header `x-amz-server-side-encryption-bucket-key-enabled"))?
        );
        output = output.set_checksum_crc32(
            crate::protocol_serde::shape_put_object_output::de_checksum_crc32_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ChecksumCRC32 from header `x-amz-checksum-crc32"))?
        );
        output = output.set_checksum_crc32_c(
            crate::protocol_serde::shape_put_object_output::de_checksum_crc32_c_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ChecksumCRC32C from header `x-amz-checksum-crc32c"))?
        );
        output = output.set_checksum_sha1(
            crate::protocol_serde::shape_put_object_output::de_checksum_sha1_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ChecksumSHA1 from header `x-amz-checksum-sha1"))?
        );
        output = output.set_checksum_sha256(
            crate::protocol_serde::shape_put_object_output::de_checksum_sha256_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ChecksumSHA256 from header `x-amz-checksum-sha256"))?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_put_object_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_expiration(
            crate::protocol_serde::shape_put_object_output::de_expiration_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse Expiration from header `x-amz-expiration"))?
        );
        output = output.set_request_charged(
            crate::protocol_serde::shape_put_object_output::de_request_charged_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged"))?
        );
        output = output.set_sse_customer_algorithm(
            crate::protocol_serde::shape_put_object_output::de_sse_customer_algorithm_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse SSECustomerAlgorithm from header `x-amz-server-side-encryption-customer-algorithm"))?
        );
        output = output.set_sse_customer_key_md5(
            crate::protocol_serde::shape_put_object_output::de_sse_customer_key_md5_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse SSECustomerKeyMD5 from header `x-amz-server-side-encryption-customer-key-MD5"))?
        );
        output = output.set_ssekms_encryption_context(
            crate::protocol_serde::shape_put_object_output::de_ssekms_encryption_context_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse SSEKMSEncryptionContext from header `x-amz-server-side-encryption-context"))?
        );
        output = output.set_ssekms_key_id(
            crate::protocol_serde::shape_put_object_output::de_ssekms_key_id_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse SSEKMSKeyId from header `x-amz-server-side-encryption-aws-kms-key-id"))?
        );
        output = output.set_server_side_encryption(
            crate::protocol_serde::shape_put_object_output::de_server_side_encryption_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse ServerSideEncryption from header `x-amz-server-side-encryption"))?
        );
        output = output.set_version_id(
            crate::protocol_serde::shape_put_object_output::de_version_id_header(response.headers())
                                    .map_err(|_|crate::error::PutObjectError::unhandled("Failed to parse VersionId from header `x-amz-version-id"))?
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string));
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

