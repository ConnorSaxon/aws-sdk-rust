// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_service_specific_credentials_list_type(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::ServiceSpecificCredentialMetadata>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("member") /* member com.amazonaws.iam#ServiceSpecificCredentialsListType$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_service_specific_credential_metadata::de_service_specific_credential_metadata(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

