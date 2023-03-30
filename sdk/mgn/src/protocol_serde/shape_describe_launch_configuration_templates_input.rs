// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_launch_configuration_templates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeLaunchConfigurationTemplatesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.launch_configuration_template_i_ds {
        let mut array_2 = object.key("launchConfigurationTemplateIDs").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    Ok(())
}

