// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_notebook_instance_input(input: &crate::input::StopNotebookInstanceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_notebook_instance_input::ser_stop_notebook_instance_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_notebook_instance_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopNotebookInstanceOutput, crate::error::StopNotebookInstanceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StopNotebookInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::StopNotebookInstanceError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_notebook_instance_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopNotebookInstanceOutput, crate::error::StopNotebookInstanceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::stop_notebook_instance_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

