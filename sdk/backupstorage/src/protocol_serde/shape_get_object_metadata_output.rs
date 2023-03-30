// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_metadata_blob_payload(body: &mut aws_smithy_http::body::SdkBody) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetObjectMetadataError> {
    // replace the body with an empty body
                let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
                Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub(crate) fn de_metadata_blob_checksum_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_metadata_blob_checksum_algorithm_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<crate::model::DataChecksumAlgorithm>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-checksum-algorithm").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_metadata_blob_length_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-data-length").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_1.len() > 1 {
                                Err(aws_smithy_http::header::ParseError::new(format!("expected one item but found {}", var_1.len())))
                            } else {
                                let mut var_1 = var_1;
                                Ok(var_1.pop())
                            }
}

pub(crate) fn de_metadata_string_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-metadata-string").iter();
    aws_smithy_http::header::one_or_none(headers)
}

