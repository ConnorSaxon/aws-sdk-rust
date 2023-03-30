// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_accept_ranges_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("accept-ranges").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn de_body_payload(body: &mut aws_smithy_http::body::SdkBody) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetObjectError> {
    // replace the body with an empty body
                let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
                Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub(crate) fn de_bucket_key_enabled_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<bool>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-bucket-key-enabled").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_1.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_1.len())))
                            } else {
                                let mut var_1 = var_1;
                                Ok(var_1.pop())
                            }
}

pub(crate) fn de_cache_control_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Cache-Control").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_checksum_crc32_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum-crc32").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_checksum_crc32_c_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum-crc32c").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_checksum_sha1_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum-sha1").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_checksum_sha256_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum-sha256").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_disposition_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Disposition").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_encoding_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Encoding").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_language_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Language").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_length_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_2 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_2.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_2.len())))
                            } else {
                                let mut var_2 = var_2;
                                Ok(var_2.pop())
                            }
}

pub(crate) fn de_content_range_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Range").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_type_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_delete_marker_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<bool>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-delete-marker").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_3.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_3.len())))
                            } else {
                                let mut var_3 = var_3;
                                Ok(var_3.pop())
                            }
}

pub(crate) fn de_e_tag_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_expiration_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-expiration").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_expires_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<aws_smithy_types::DateTime>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Expires").iter();
    let var_4: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(headers, aws_smithy_types::date_time::Format::HttpDate)?;
    if var_4.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_4.len())))
                            } else {
                                let mut var_4 = var_4;
                                Ok(var_4.pop())
                            }
}

pub(crate) fn de_last_modified_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<aws_smithy_types::DateTime>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_5: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(headers, aws_smithy_types::date_time::Format::HttpDate)?;
    if var_5.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_5.len())))
                            } else {
                                let mut var_5 = var_5;
                                Ok(var_5.pop())
                            }
}

pub(crate) fn de_metadata_prefix_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>, aws_smithy_http::header::ParseError> {
    let headers = aws_smithy_http::header::headers_for_prefix(header_map, "x-amz-meta-");
                        let out: std::result::Result<_, _> = headers.map(|(key, header_name)| {
                            let values = header_map.get_all(header_name);
                            crate::protocol_serde::shape_get_object_output::de_metadata_inner(values.iter()).map(|v| (key.to_string(), v.expect(
                                "we have checked there is at least one value for this header name; please file a bug report under https://github.com/awslabs/smithy-rs/issues"
                            )))
                        }).collect();
    out.map(Some)
}

pub(crate) fn de_missing_meta_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-missing-meta").iter();
    let var_6 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_6.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_6.len())))
                            } else {
                                let mut var_6 = var_6;
                                Ok(var_6.pop())
                            }
}

pub(crate) fn de_object_lock_legal_hold_status_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::ObjectLockLegalHoldStatus>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-object-lock-legal-hold").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_object_lock_mode_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::ObjectLockMode>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-object-lock-mode").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_object_lock_retain_until_date_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<aws_smithy_types::DateTime>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-object-lock-retain-until-date").iter();
    let var_7: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(headers, aws_smithy_types::date_time::Format::DateTime)?;
    if var_7.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_7.len())))
                            } else {
                                let mut var_7 = var_7;
                                Ok(var_7.pop())
                            }
}

pub(crate) fn de_parts_count_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-mp-parts-count").iter();
    let var_8 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_8.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_8.len())))
                            } else {
                                let mut var_8 = var_8;
                                Ok(var_8.pop())
                            }
}

pub(crate) fn de_replication_status_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::ReplicationStatus>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-replication-status").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_request_charged_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::RequestCharged>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-request-charged").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_restore_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-restore").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_sse_customer_algorithm_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-customer-algorithm").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_sse_customer_key_md5_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-customer-key-MD5").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_ssekms_key_id_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-aws-kms-key-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_server_side_encryption_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::ServerSideEncryption>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_storage_class_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::StorageClass>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-storage-class").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_tag_count_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-tagging-count").iter();
    let var_9 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_9.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_9.len())))
                            } else {
                                let mut var_9 = var_9;
                                Ok(var_9.pop())
                            }
}

pub(crate) fn de_version_id_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-version-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_website_redirect_location_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-website-redirect-location").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn de_metadata_inner(headers: http::header::ValueIter<http::HeaderValue>) -> std::result::Result<Option<std::string::String>, aws_smithy_http::header::ParseError> {
    aws_smithy_http::header::one_or_none(headers)
}

