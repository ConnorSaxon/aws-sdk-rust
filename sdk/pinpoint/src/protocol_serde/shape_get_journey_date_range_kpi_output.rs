// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_journey_date_range_kpi_response_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::JourneyDateRangeKpiResponse>, crate::error::GetJourneyDateRangeKpiError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_journey_date_range_kpi_response::de_journey_date_range_kpi_response_payload(body).map_err(crate::error::GetJourneyDateRangeKpiError::unhandled)
    }).transpose()
}

