// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_service_endpoint(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ServiceEndpoint, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ServiceEndpoint::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Endpoint") /* Endpoint com.amazonaws.cloudsearch#ServiceEndpoint$Endpoint */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_endpoint(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

