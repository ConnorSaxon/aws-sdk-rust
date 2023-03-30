// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_invalidation(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Invalidation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Invalidation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudfront#Invalidation$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.cloudfront#Invalidation$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            s if s.matches("CreateTime") /* CreateTime com.amazonaws.cloudfront#Invalidation$CreateTime */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudfront#timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_3);
            }
            ,
            s if s.matches("InvalidationBatch") /* InvalidationBatch com.amazonaws.cloudfront#Invalidation$InvalidationBatch */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_invalidation_batch::de_invalidation_batch(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_invalidation_batch(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

