// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_template_action_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutTemplateActionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_id {
        object.key("actionID").string(var_1.as_str());
    }
    if let Some(var_2) = &input.action_name {
        object.key("actionName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.active {
        object.key("active").boolean(*var_3);
    }
    if let Some(var_4) = &input.document_identifier {
        object.key("documentIdentifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.document_version {
        object.key("documentVersion").string(var_5.as_str());
    }
    if let Some(var_6) = &input.launch_configuration_template_id {
        object.key("launchConfigurationTemplateID").string(var_6.as_str());
    }
    if let Some(var_7) = &input.must_succeed_for_cutover {
        object.key("mustSucceedForCutover").boolean(*var_7);
    }
    if let Some(var_8) = &input.operating_system {
        object.key("operatingSystem").string(var_8.as_str());
    }
     {
        object.key("order").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.order).into()));
    }
    if let Some(var_9) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_10 = object.key("parameters").start_object();
        for (key_11, value_12) in var_9 {
             {
                let mut array_13 = object_10.key(key_11.as_str()).start_array();
                for item_14 in value_12 {
                     {
                        #[allow(unused_mut)]
                        let mut object_15 = array_13.value().start_object();
                        crate::protocol_serde::shape_ssm_parameter_store_parameter::ser_ssm_parameter_store_parameter(&mut object_15, item_14)?;
                        object_15.finish();
                    }
                }
                array_13.finish();
            }
        }
        object_10.finish();
    }
    if input.timeout_seconds != 0 {
        object.key("timeoutSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timeout_seconds).into()));
    }
    Ok(())
}

