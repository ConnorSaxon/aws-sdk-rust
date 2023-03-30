// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pivot_table_field_wells(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PivotTableFieldWells) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pivot_table_aggregated_field_wells {
        #[allow(unused_mut)]
        let mut object_2 = object.key("PivotTableAggregatedFieldWells").start_object();
        crate::protocol_serde::shape_pivot_table_aggregated_field_wells::ser_pivot_table_aggregated_field_wells(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_pivot_table_field_wells<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PivotTableFieldWells>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::pivot_table_field_wells::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PivotTableAggregatedFieldWells" => {
                                builder = builder.set_pivot_table_aggregated_field_wells(
                                    crate::protocol_serde::shape_pivot_table_aggregated_field_wells::de_pivot_table_aggregated_field_wells(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

