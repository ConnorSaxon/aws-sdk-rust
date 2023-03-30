// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_export_task_input_input(input: &crate::input::CancelExportTaskInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CancelExportTask", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ExportTaskId");
    if let Some(var_2) = &input.export_task_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

