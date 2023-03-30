// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_legal_hold_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::ObjectLockLegalHold>, crate::error::GetObjectLegalHoldError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_get_object_legal_hold_output::de_legal_hold(body).map_err(crate::error::GetObjectLegalHoldError::unhandled)
    }).transpose()
}

pub fn de_legal_hold(inp: &[u8]) -> Result<crate::model::ObjectLockLegalHold, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        let start_el = decoder.start_el();
                        if !(start_el.matches("ObjectLockLegalHold")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ObjectLockLegalHold got {:?}", start_el)))
                        }
    crate::protocol_serde::shape_object_lock_legal_hold::de_object_lock_legal_hold(&mut decoder)
}

