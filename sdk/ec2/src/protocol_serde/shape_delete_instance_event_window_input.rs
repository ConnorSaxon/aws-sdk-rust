// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_instance_event_window_input_input(input: &crate::input::DeleteInstanceEventWindowInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteInstanceEventWindow", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ForceDelete");
    if let Some(var_4) = &input.force_delete {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("InstanceEventWindowId");
    if let Some(var_6) = &input.instance_event_window_id {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

