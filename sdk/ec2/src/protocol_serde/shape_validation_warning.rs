// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_validation_warning(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ValidationWarning, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ValidationWarning::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("errorSet") /* Errors com.amazonaws.ec2#ValidationWarning$Errors */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_error_set::de_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_errors(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

