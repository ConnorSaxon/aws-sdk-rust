// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_index_field_status(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::IndexFieldStatus, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::IndexFieldStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Options") /* Options com.amazonaws.cloudsearch#IndexFieldStatus$Options */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_index_field::de_index_field(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_options(var_1);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.cloudsearch#IndexFieldStatus$Status */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_option_status::de_option_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

