// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_data_query(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::DataQuery) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Id");
    if let Some(var_2) = &input.id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Source");
    if let Some(var_4) = &input.source {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Destination");
    if let Some(var_6) = &input.destination {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Metric");
    if let Some(var_8) = &input.metric {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Statistic");
    if let Some(var_10) = &input.statistic {
        scope_9.string(var_10.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Period");
    if let Some(var_12) = &input.period {
        scope_11.string(var_12.as_str());
    }
    Ok(())
}

