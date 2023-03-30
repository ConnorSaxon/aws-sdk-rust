// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_frame_metric_data_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetFrameMetricDataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.frame_metrics {
        let mut array_2 = object.key("frameMetrics").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_frame_metric::ser_frame_metric(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

