// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_reserved_instances_exchange_quote_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetReservedInstancesExchangeQuoteOutput, crate::error::GetReservedInstancesExchangeQuoteError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetReservedInstancesExchangeQuoteError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::GetReservedInstancesExchangeQuoteError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_reserved_instances_exchange_quote_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetReservedInstancesExchangeQuoteOutput, crate::error::GetReservedInstancesExchangeQuoteError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_reserved_instances_exchange_quote_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_reserved_instances_exchange_quote::de_get_reserved_instances_exchange_quote(response.body().as_ref(), output).map_err(crate::error::GetReservedInstancesExchangeQuoteError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_reserved_instances_exchange_quote(inp: &[u8], mut builder: crate::output::get_reserved_instances_exchange_quote_output::Builder) -> Result<crate::output::get_reserved_instances_exchange_quote_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GetReservedInstancesExchangeQuoteResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GetReservedInstancesExchangeQuoteResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$CurrencyCode */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_currency_code(var_1);
            }
            ,
            s if s.matches("isValidExchange") /* IsValidExchange com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$IsValidExchange */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_valid_exchange(var_2);
            }
            ,
            s if s.matches("outputReservedInstancesWillExpireAt") /* OutputReservedInstancesWillExpireAt com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$OutputReservedInstancesWillExpireAt */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_output_reserved_instances_will_expire_at(var_3);
            }
            ,
            s if s.matches("paymentDue") /* PaymentDue com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$PaymentDue */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_payment_due(var_4);
            }
            ,
            s if s.matches("reservedInstanceValueRollup") /* ReservedInstanceValueRollup com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$ReservedInstanceValueRollup */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_reservation_value::de_reservation_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_reserved_instance_value_rollup(var_5);
            }
            ,
            s if s.matches("reservedInstanceValueSet") /* ReservedInstanceValueSet com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$ReservedInstanceValueSet */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_reserved_instance_reservation_value_set::de_reserved_instance_reservation_value_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_reserved_instance_value_set(var_6);
            }
            ,
            s if s.matches("targetConfigurationValueRollup") /* TargetConfigurationValueRollup com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$TargetConfigurationValueRollup */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_reservation_value::de_reservation_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_configuration_value_rollup(var_7);
            }
            ,
            s if s.matches("targetConfigurationValueSet") /* TargetConfigurationValueSet com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$TargetConfigurationValueSet */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_target_reservation_value_set::de_target_reservation_value_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_configuration_value_set(var_8);
            }
            ,
            s if s.matches("validationFailureReason") /* ValidationFailureReason com.amazonaws.ec2.synthetic#GetReservedInstancesExchangeQuoteOutput$ValidationFailureReason */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_validation_failure_reason(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

