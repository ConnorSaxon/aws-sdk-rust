// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input_serialization(input: &crate::model::InputSerialization, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.csv {
        let inner_writer = scope.start_el("CSV");
        crate::protocol_serde::shape_csv_input::ser_csv_input(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.compression_type {
        let mut inner_writer = scope.start_el("CompressionType").finish();
        inner_writer.data(
            var_2.as_str()
        );
    }
    if let Some(var_3) = &input.json {
        let inner_writer = scope.start_el("JSON");
        crate::protocol_serde::shape_json_input::ser_json_input(var_3, inner_writer)?
    }
    if let Some(_var_4) = &input.parquet {
        scope.start_el("Parquet").finish();
    }
    scope.finish();
    Ok(())
}

