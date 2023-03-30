// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_host_reservation_purchase_preview_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetHostReservationPurchasePreviewOutput, crate::error::GetHostReservationPurchasePreviewError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetHostReservationPurchasePreviewError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::GetHostReservationPurchasePreviewError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_host_reservation_purchase_preview_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetHostReservationPurchasePreviewOutput, crate::error::GetHostReservationPurchasePreviewError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_host_reservation_purchase_preview_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_host_reservation_purchase_preview::de_get_host_reservation_purchase_preview(response.body().as_ref(), output).map_err(crate::error::GetHostReservationPurchasePreviewError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_host_reservation_purchase_preview(inp: &[u8], mut builder: crate::output::get_host_reservation_purchase_preview_output::Builder) -> Result<crate::output::get_host_reservation_purchase_preview_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GetHostReservationPurchasePreviewResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GetHostReservationPurchasePreviewResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2.synthetic#GetHostReservationPurchasePreviewOutput$CurrencyCode */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::CurrencyCodeValues, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::CurrencyCodeValues::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_currency_code(var_1);
            }
            ,
            s if s.matches("purchase") /* Purchase com.amazonaws.ec2.synthetic#GetHostReservationPurchasePreviewOutput$Purchase */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_purchase_set::de_purchase_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_purchase(var_2);
            }
            ,
            s if s.matches("totalHourlyPrice") /* TotalHourlyPrice com.amazonaws.ec2.synthetic#GetHostReservationPurchasePreviewOutput$TotalHourlyPrice */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_total_hourly_price(var_3);
            }
            ,
            s if s.matches("totalUpfrontPrice") /* TotalUpfrontPrice com.amazonaws.ec2.synthetic#GetHostReservationPurchasePreviewOutput$TotalUpfrontPrice */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_total_upfront_price(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

