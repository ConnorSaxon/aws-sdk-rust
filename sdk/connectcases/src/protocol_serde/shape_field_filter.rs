// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_field_filter(object_1: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FieldFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::FieldFilter::EqualTo(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_1.key("equalTo").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::FieldFilter::Contains(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_2 = object_1.key("contains").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_2, inner)?;
                object_2.finish();
            }
        },
        crate::model::FieldFilter::GreaterThan(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_3 = object_1.key("greaterThan").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_3, inner)?;
                object_3.finish();
            }
        },
        crate::model::FieldFilter::GreaterThanOrEqualTo(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_4 = object_1.key("greaterThanOrEqualTo").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_4, inner)?;
                object_4.finish();
            }
        },
        crate::model::FieldFilter::LessThan(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_1.key("lessThan").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_5, inner)?;
                object_5.finish();
            }
        },
        crate::model::FieldFilter::LessThanOrEqualTo(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_6 = object_1.key("lessThanOrEqualTo").start_object();
                crate::protocol_serde::shape_field_value::ser_field_value(&mut object_6, inner)?;
                object_6.finish();
            }
        },
        crate::model::FieldFilter::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("FieldFilter"))
    }
    Ok(())
}

