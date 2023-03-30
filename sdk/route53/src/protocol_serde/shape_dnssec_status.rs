// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_dnssec_status(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DnssecStatus, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DnssecStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ServeSignature") /* ServeSignature com.amazonaws.route53#DNSSECStatus$ServeSignature */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_serve_signature(var_1);
            }
            ,
            s if s.matches("StatusMessage") /* StatusMessage com.amazonaws.route53#DNSSECStatus$StatusMessage */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

