// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_access_control_expose_headers_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<std::string::String>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Header") /* member com.amazonaws.cloudfront#AccessControlExposeHeadersList$member */ =>  {
                out.push(
                    Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

