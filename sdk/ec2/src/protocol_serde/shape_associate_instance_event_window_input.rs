// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_instance_event_window_input_input(input: &crate::input::AssociateInstanceEventWindowInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AssociateInstanceEventWindow", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceEventWindowId");
    if let Some(var_4) = &input.instance_event_window_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AssociationTarget");
    if let Some(var_6) = &input.association_target {
        crate::protocol_serde::shape_instance_event_window_association_request::ser_instance_event_window_association_request(scope_5, var_6)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

