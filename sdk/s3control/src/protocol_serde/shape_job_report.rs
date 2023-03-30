// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_report(input: &crate::model::JobReport, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.bucket {
        let mut inner_writer = scope.start_el("Bucket").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    if let Some(var_2) = &input.format {
        let mut inner_writer = scope.start_el("Format").finish();
        inner_writer.data(
            var_2.as_str()
        );
    }
     {
        let mut inner_writer = scope.start_el("Enabled").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.enabled).encode()
        );
    }
    if let Some(var_3) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(
            var_3.as_str()
        );
    }
    if let Some(var_4) = &input.report_scope {
        let mut inner_writer = scope.start_el("ReportScope").finish();
        inner_writer.data(
            var_4.as_str()
        );
    }
    scope.finish();
    Ok(())
}

pub fn de_job_report(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::JobReport, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::JobReport::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3control#JobReport$Bucket */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_5);
            }
            ,
            s if s.matches("Format") /* Format com.amazonaws.s3control#JobReport$Format */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::model::JobReportFormat, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::JobReportFormat::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_format(var_6);
            }
            ,
            s if s.matches("Enabled") /* Enabled com.amazonaws.s3control#JobReport$Enabled */ =>  {
                let var_7 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enabled(var_7);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3control#JobReport$Prefix */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_8);
            }
            ,
            s if s.matches("ReportScope") /* ReportScope com.amazonaws.s3control#JobReport$ReportScope */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::model::JobReportScope, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::JobReportScope::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_report_scope(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

