// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_technical_cue_detection_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StartTechnicalCueDetectionFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.min_segment_confidence {
        object.key("MinSegmentConfidence").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_1).into()));
    }
    if let Some(var_2) = &input.black_frame {
        #[allow(unused_mut)]
        let mut object_3 = object.key("BlackFrame").start_object();
        crate::protocol_serde::shape_black_frame::ser_black_frame(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

