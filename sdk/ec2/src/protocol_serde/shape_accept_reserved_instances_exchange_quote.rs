// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_reserved_instances_exchange_quote_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcceptReservedInstancesExchangeQuoteOutput, crate::error::AcceptReservedInstancesExchangeQuoteError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AcceptReservedInstancesExchangeQuoteError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::AcceptReservedInstancesExchangeQuoteError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_reserved_instances_exchange_quote_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcceptReservedInstancesExchangeQuoteOutput, crate::error::AcceptReservedInstancesExchangeQuoteError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::accept_reserved_instances_exchange_quote_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_accept_reserved_instances_exchange_quote::de_accept_reserved_instances_exchange_quote(response.body().as_ref(), output).map_err(crate::error::AcceptReservedInstancesExchangeQuoteError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_accept_reserved_instances_exchange_quote(inp: &[u8], mut builder: crate::output::accept_reserved_instances_exchange_quote_output::Builder) -> Result<crate::output::accept_reserved_instances_exchange_quote_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("AcceptReservedInstancesExchangeQuoteResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected AcceptReservedInstancesExchangeQuoteResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("exchangeId") /* ExchangeId com.amazonaws.ec2.synthetic#AcceptReservedInstancesExchangeQuoteOutput$ExchangeId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_exchange_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

