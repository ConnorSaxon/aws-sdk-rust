// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_job_timers(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::JobTimers, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::JobTimers::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ElapsedTimeInActiveSeconds") /* ElapsedTimeInActiveSeconds com.amazonaws.s3control#JobTimers$ElapsedTimeInActiveSeconds */ =>  {
                let var_1 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3control#JobTimeInStateSeconds`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_elapsed_time_in_active_seconds(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

