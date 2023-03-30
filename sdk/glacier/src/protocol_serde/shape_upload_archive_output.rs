// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_archive_id_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-archive-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_checksum_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-sha256-tree-hash").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_location_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Location").iter();
    aws_smithy_http::header::one_or_none(headers)
}

