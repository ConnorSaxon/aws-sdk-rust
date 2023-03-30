// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_metric(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Metric) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Namespace");
    if let Some(var_2) = &input.namespace {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MetricName");
    if let Some(var_4) = &input.metric_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Dimensions");
    if let Some(var_6) = &input.dimensions {
        let mut list_8 = scope_5.start_list(false, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_metric_dimension::ser_metric_dimension(entry_9, item_7)?;
        }
        list_8.finish();
    }
    Ok(())
}

pub fn de_metric(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Metric, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Metric::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Namespace") /* Namespace com.amazonaws.autoscaling#Metric$Namespace */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_namespace(var_10);
            }
            ,
            s if s.matches("MetricName") /* MetricName com.amazonaws.autoscaling#Metric$MetricName */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_metric_name(var_11);
            }
            ,
            s if s.matches("Dimensions") /* Dimensions com.amazonaws.autoscaling#Metric$Dimensions */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_metric_dimensions::de_metric_dimensions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dimensions(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

