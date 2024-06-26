// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn too_many_requests_exception_correct_errors(
    mut builder: crate::types::error::builders::TooManyRequestsExceptionBuilder,
) -> crate::types::error::builders::TooManyRequestsExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn contact_information_correct_errors(
    mut builder: crate::types::builders::ContactInformationBuilder,
) -> crate::types::builders::ContactInformationBuilder {
    if builder.full_name.is_none() {
        builder.full_name = Some(Default::default())
    }
    if builder.address_line1.is_none() {
        builder.address_line1 = Some(Default::default())
    }
    if builder.city.is_none() {
        builder.city = Some(Default::default())
    }
    if builder.postal_code.is_none() {
        builder.postal_code = Some(Default::default())
    }
    if builder.country_code.is_none() {
        builder.country_code = Some(Default::default())
    }
    if builder.phone_number.is_none() {
        builder.phone_number = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_field_correct_errors(
    mut builder: crate::types::builders::ValidationExceptionFieldBuilder,
) -> crate::types::builders::ValidationExceptionFieldBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}
