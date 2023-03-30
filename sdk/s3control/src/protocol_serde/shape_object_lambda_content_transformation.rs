// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_object_lambda_content_transformation(input: &crate::model::ObjectLambdaContentTransformation, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    let mut scope_writer = writer.finish();
    match input {
        crate::model::ObjectLambdaContentTransformation::AwsLambda(inner) =>
             {
                let inner_writer = scope_writer.start_el("AwsLambda");
                crate::protocol_serde::shape_aws_lambda_transformation::ser_aws_lambda_transformation(inner, inner_writer)?
            }
        ,
        crate::model::ObjectLambdaContentTransformation::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("ObjectLambdaContentTransformation"))
    }
    Ok(())
}

pub fn de_object_lambda_content_transformation(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ObjectLambdaContentTransformation, aws_smithy_xml::decode::XmlDecodeError> {
    let mut base: Option<crate::model::ObjectLambdaContentTransformation> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AwsLambda") /* AwsLambda com.amazonaws.s3control#ObjectLambdaContentTransformation$AwsLambda */ =>  {
                let tmp =
                    crate::protocol_serde::shape_aws_lambda_transformation::de_aws_lambda_transformation(&mut tag)
                    ?
                ;
                base = Some(crate::model::ObjectLambdaContentTransformation::AwsLambda(tmp));
            }
            ,
            _unknown => base = Some(crate::model::ObjectLambdaContentTransformation::Unknown),
        }
    }
    base.ok_or_else(||aws_smithy_xml::decode::XmlDecodeError::custom("expected union, got nothing"))
}

