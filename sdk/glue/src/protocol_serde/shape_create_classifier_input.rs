// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_classifier_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateClassifierInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.grok_classifier {
        #[allow(unused_mut)]
        let mut object_2 = object.key("GrokClassifier").start_object();
        crate::protocol_serde::shape_create_grok_classifier_request::ser_create_grok_classifier_request(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.xml_classifier {
        #[allow(unused_mut)]
        let mut object_4 = object.key("XMLClassifier").start_object();
        crate::protocol_serde::shape_create_xml_classifier_request::ser_create_xml_classifier_request(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.json_classifier {
        #[allow(unused_mut)]
        let mut object_6 = object.key("JsonClassifier").start_object();
        crate::protocol_serde::shape_create_json_classifier_request::ser_create_json_classifier_request(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.csv_classifier {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CsvClassifier").start_object();
        crate::protocol_serde::shape_create_csv_classifier_request::ser_create_csv_classifier_request(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

