// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_content_type_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_e_tag_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn de_function_code_payload(body: &[u8]) -> std::result::Result<std::option::Option<aws_smithy_types::Blob>, crate::error::GetFunctionError> {
    (!body.is_empty()).then(||{
        Ok(aws_smithy_types::Blob::new(body))
    }).transpose()
}

