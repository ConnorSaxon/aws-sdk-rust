// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_describe_db_log_files_details(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DescribeDbLogFilesDetails, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DescribeDbLogFilesDetails::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LogFileName") /* LogFileName com.amazonaws.rds#DescribeDBLogFilesDetails$LogFileName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_log_file_name(var_1);
            }
            ,
            s if s.matches("LastWritten") /* LastWritten com.amazonaws.rds#DescribeDBLogFilesDetails$LastWritten */ =>  {
                let var_2 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.rds#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_last_written(var_2);
            }
            ,
            s if s.matches("Size") /* Size com.amazonaws.rds#DescribeDBLogFilesDetails$Size */ =>  {
                let var_3 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.rds#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_size(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

