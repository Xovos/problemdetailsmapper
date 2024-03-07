use http::StatusCode;

use crate::builders::problemdetails_builder::ProblemDetailsBuilder;

#[test]
fn build_bad_request_should_run_as_expected() {
    const TITLE: &'static str = "Bad Request";
    const DETAIL: &'static str = "test";

    let result = ProblemDetailsBuilder::build_bad_request(Some(DETAIL), None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::BAD_REQUEST));
}

#[test]
fn build_bad_request_with_none_detail_should_use_default() {
    const TITLE: &'static str = "Bad Request";
    const DETAIL: &'static str = "The request is invalid or malformed.";

    let result = ProblemDetailsBuilder::build_bad_request(None::<String>, None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::BAD_REQUEST));
}

#[test]
fn build_not_found_should_run_as_expected() {
    const TITLE: &'static str = "Resource not found";
    const DETAIL: &'static str = "test";

    let result = ProblemDetailsBuilder::build_not_found(Some(DETAIL), None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::NOT_FOUND));
}

#[test]
fn build_not_found_with_none_detail_should_use_default() {
    const TITLE: &'static str = "Resource not found";
    const DETAIL: &'static str = "The requested resource could not be found on the server.";

    let result = ProblemDetailsBuilder::build_not_found(None::<String>, None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::NOT_FOUND));
}

#[test]
fn build_server_error_should_run_as_expected() {
    const TITLE: &'static str = "Internal Server Error";
    const DETAIL: &'static str = "test";

    let result = ProblemDetailsBuilder::build_server_error(Some(DETAIL), None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
}

#[test]
fn build_server_error_with_none_detail_should_use_default() {
    const TITLE: &'static str = "Internal Server Error";
    const DETAIL: &'static str = "An unexpected error occurred on the server.";

    let result = ProblemDetailsBuilder::build_server_error(None::<String>, None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
}

#[test]
fn build_should_run_as_expected() {
    const TITLE: &'static str = "TEST_TITLE";
    const DETAIL: &'static str = "TEST_DETAIL";
    const STATUS: StatusCode = StatusCode::FORBIDDEN;

    let result = ProblemDetailsBuilder::build(STATUS, TITLE.to_string(), DETAIL.to_string(), None::<String>, None::<String>);

    assert_eq!(result.title, Some(TITLE.to_string()));
    assert_eq!(result.detail, Some(DETAIL.to_string()));
    assert_eq!(result.status, Some(STATUS));
}