// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_activities_response_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::ActivitiesResponse>, crate::error::GetCampaignActivitiesError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_activities_response::de_activities_response_payload(body).map_err(crate::error::GetCampaignActivitiesError::unhandled)
    }).transpose()
}

