// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_function_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FunctionRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.required_properties {
        let mut array_2 = object.key("requiredProperties").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.scope {
        object.key("scope").string(var_4.as_str());
    }
    if let Some(var_5) = &input.implemented_by {
        #[allow(unused_mut)]
        let mut object_6 = object.key("implementedBy").start_object();
        crate::protocol_serde::shape_data_connector::ser_data_connector(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

