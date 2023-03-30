// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_set_import_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataSetImportItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data_set {
        #[allow(unused_mut)]
        let mut object_2 = object.key("dataSet").start_object();
        crate::protocol_serde::shape_data_set::ser_data_set(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.external_location {
        #[allow(unused_mut)]
        let mut object_4 = object.key("externalLocation").start_object();
        crate::protocol_serde::shape_external_location::ser_external_location(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

