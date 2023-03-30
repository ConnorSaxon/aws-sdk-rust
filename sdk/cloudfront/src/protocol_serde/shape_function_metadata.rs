// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_function_metadata(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::FunctionMetadata, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::FunctionMetadata::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("FunctionARN") /* FunctionARN com.amazonaws.cloudfront#FunctionMetadata$FunctionARN */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_function_arn(var_1);
            }
            ,
            s if s.matches("Stage") /* Stage com.amazonaws.cloudfront#FunctionMetadata$Stage */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::FunctionStage, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::FunctionStage::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_stage(var_2);
            }
            ,
            s if s.matches("CreatedTime") /* CreatedTime com.amazonaws.cloudfront#FunctionMetadata$CreatedTime */ =>  {
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
                builder = builder.set_created_time(var_3);
            }
            ,
            s if s.matches("LastModifiedTime") /* LastModifiedTime com.amazonaws.cloudfront#FunctionMetadata$LastModifiedTime */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudfront#timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified_time(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

