// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_conflicting_aliases_list_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::ConflictingAliasesList>, crate::error::ListConflictingAliasesError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_list_conflicting_aliases_output::de_conflicting_aliases_list(body).map_err(crate::error::ListConflictingAliasesError::unhandled)
    }).transpose()
}

pub fn de_conflicting_aliases_list(inp: &[u8]) -> Result<crate::model::ConflictingAliasesList, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        let start_el = decoder.start_el();
                        if !(start_el.matches("ConflictingAliasesList")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ConflictingAliasesList got {:?}", start_el)))
                        }
    crate::protocol_serde::shape_conflicting_aliases_list::de_conflicting_aliases_list(&mut decoder)
}

