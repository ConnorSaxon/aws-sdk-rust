// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_appflow_integration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AppflowIntegration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.flow_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FlowDefinition").start_object();
        crate::protocol_serde::shape_flow_definition::ser_flow_definition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.batches {
        let mut array_4 = object.key("Batches").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_batch::ser_batch(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

