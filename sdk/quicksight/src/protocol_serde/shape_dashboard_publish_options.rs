// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dashboard_publish_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DashboardPublishOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ad_hoc_filtering_option {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AdHocFilteringOption").start_object();
        crate::protocol_serde::shape_ad_hoc_filtering_option::ser_ad_hoc_filtering_option(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.export_to_csv_option {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ExportToCSVOption").start_object();
        crate::protocol_serde::shape_export_to_csv_option::ser_export_to_csv_option(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.sheet_controls_option {
        #[allow(unused_mut)]
        let mut object_6 = object.key("SheetControlsOption").start_object();
        crate::protocol_serde::shape_sheet_controls_option::ser_sheet_controls_option(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.visual_publish_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("VisualPublishOptions").start_object();
        crate::protocol_serde::shape_dashboard_visual_publish_options::ser_dashboard_visual_publish_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

