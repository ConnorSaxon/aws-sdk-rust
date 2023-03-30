// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_analyzer_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNetworkAnalyzerConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.trace_content {
        #[allow(unused_mut)]
        let mut object_9 = object.key("TraceContent").start_object();
        crate::protocol_serde::shape_trace_content::ser_trace_content(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.wireless_devices {
        let mut array_11 = object.key("WirelessDevices").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.wireless_gateways {
        let mut array_14 = object.key("WirelessGateways").start_array();
        for item_15 in var_13 {
             {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    Ok(())
}

