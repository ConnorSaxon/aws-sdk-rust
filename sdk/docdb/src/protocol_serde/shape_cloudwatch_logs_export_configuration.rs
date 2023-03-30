// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_cloudwatch_logs_export_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CloudwatchLogsExportConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnableLogTypes");
    if let Some(var_2) = &input.enable_log_types {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DisableLogTypes");
    if let Some(var_7) = &input.disable_log_types {
        let mut list_9 = scope_6.start_list(false, None);
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    Ok(())
}

