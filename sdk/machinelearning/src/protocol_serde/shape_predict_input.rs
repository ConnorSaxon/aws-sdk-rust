// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_predict_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PredictInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ml_model_id {
        object.key("MLModelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.record {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Record").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.predict_endpoint {
        object.key("PredictEndpoint").string(var_6.as_str());
    }
    Ok(())
}

