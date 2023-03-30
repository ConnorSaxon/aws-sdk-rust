// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_metric_stream_filter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::MetricStreamFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Namespace");
    if let Some(var_2) = &input.namespace {
        scope_1.string(var_2);
    }
    Ok(())
}

pub fn de_metric_stream_filter(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::MetricStreamFilter, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::MetricStreamFilter::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Namespace") /* Namespace com.amazonaws.cloudwatch#MetricStreamFilter$Namespace */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_namespace(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

