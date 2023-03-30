// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_findings_statistics_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetFindingsStatisticsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.finding_criteria {
        #[allow(unused_mut)]
        let mut object_2 = object.key("findingCriteria").start_object();
        crate::protocol_serde::shape_finding_criteria::ser_finding_criteria(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.finding_statistic_types {
        let mut array_4 = object.key("findingStatisticTypes").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

