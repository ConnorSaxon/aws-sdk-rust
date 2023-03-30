// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_predictor_backtest_export_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePredictorBacktestExportJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.predictor_backtest_export_job_name {
        object.key("PredictorBacktestExportJobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.predictor_arn {
        object.key("PredictorArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Destination").start_object();
        crate::protocol_serde::shape_data_destination::ser_data_destination(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.format {
        object.key("Format").string(var_9.as_str());
    }
    Ok(())
}

