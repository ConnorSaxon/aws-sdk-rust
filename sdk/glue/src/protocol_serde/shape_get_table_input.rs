// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTableInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_name {
        object.key("DatabaseName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.transaction_id {
        object.key("TransactionId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.query_as_of_time {
        object.key("QueryAsOfTime").date_time(var_5, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

