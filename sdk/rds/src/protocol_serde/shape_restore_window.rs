// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_restore_window(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::RestoreWindow, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::RestoreWindow::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("EarliestTime") /* EarliestTime com.amazonaws.rds#RestoreWindow$EarliestTime */ =>  {
                let var_1 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.rds#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_earliest_time(var_1);
            }
            ,
            s if s.matches("LatestTime") /* LatestTime com.amazonaws.rds#RestoreWindow$LatestTime */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.rds#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_latest_time(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

