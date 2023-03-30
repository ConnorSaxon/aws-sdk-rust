// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_e_tag_header(header_map: &http::HeaderMap) -> std::result::Result<std::option::Option<std::string::String>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn de_key_group_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::KeyGroup>, crate::error::GetKeyGroupError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_get_key_group_output::de_key_group(body).map_err(crate::error::GetKeyGroupError::unhandled)
    }).transpose()
}

pub fn de_key_group(inp: &[u8]) -> Result<crate::model::KeyGroup, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        let start_el = decoder.start_el();
                        if !(start_el.matches("KeyGroup")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected KeyGroup got {:?}", start_el)))
                        }
    crate::protocol_serde::shape_key_group::de_key_group(&mut decoder)
}

