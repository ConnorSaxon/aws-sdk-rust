// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_stats(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Stats, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Stats::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("BytesScanned") /* BytesScanned com.amazonaws.s3#Stats$BytesScanned */ =>  {
                let var_1 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3#BytesScanned`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_bytes_scanned(var_1);
            }
            ,
            s if s.matches("BytesProcessed") /* BytesProcessed com.amazonaws.s3#Stats$BytesProcessed */ =>  {
                let var_2 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3#BytesProcessed`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_bytes_processed(var_2);
            }
            ,
            s if s.matches("BytesReturned") /* BytesReturned com.amazonaws.s3#Stats$BytesReturned */ =>  {
                let var_3 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3#BytesReturned`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_bytes_returned(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

