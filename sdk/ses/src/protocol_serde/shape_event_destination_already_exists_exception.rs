// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn de_event_destination_already_exists_exception_xml_err(inp: &[u8], mut builder: crate::error::event_destination_already_exists_exception::Builder) -> Result<crate::error::event_destination_already_exists_exception::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    if inp.is_empty() { return Ok(builder) }
    let mut document = aws_smithy_xml::decode::Document::try_from(inp)?;
                            #[allow(unused_mut)]
                            let mut error_decoder = crate::rest_xml_wrapped_errors::error_scope(&mut document)?;
    while let Some(mut tag) = error_decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("EventDestinationName") /* EventDestinationName com.amazonaws.ses#EventDestinationAlreadyExistsException$EventDestinationName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_event_destination_name(var_1);
            }
            ,
            s if s.matches("ConfigurationSetName") /* ConfigurationSetName com.amazonaws.ses#EventDestinationAlreadyExistsException$ConfigurationSetName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_configuration_set_name(var_2);
            }
            ,
            s if s.matches("message") /* message com.amazonaws.ses#EventDestinationAlreadyExistsException$message */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

