// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_realtime_log_config_input_input(input: &crate::input::UpdateRealtimeLogConfigInput, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.arn {
        let mut inner_writer = scope.start_el("ARN").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    if let Some(var_2) = &input.end_points {
        let mut inner_writer = scope.start_el("EndPoints").finish();
        for list_item_3 in var_2 {
             {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_end_point::ser_end_point(list_item_3, inner_writer)?
            }
        }
    }
    if let Some(var_4) = &input.fields {
        let mut inner_writer = scope.start_el("Fields").finish();
        for list_item_5 in var_4 {
             {
                let mut inner_writer = inner_writer.start_el("Field").finish();
                inner_writer.data(
                    list_item_5.as_str()
                );
            }
        }
    }
    if let Some(var_6) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(
            var_6.as_str()
        );
    }
    if let Some(var_7) = &input.sampling_rate {
        let mut inner_writer = scope.start_el("SamplingRate").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_7).encode()
        );
    }
    scope.finish();
    Ok(())
}

