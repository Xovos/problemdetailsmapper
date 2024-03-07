use std::error;
use std::fmt;
use http::StatusCode;

use crate::{builders::problemdetails_builder::ProblemDetailsBuilder, errors::mapper_error::MapperError, mappers::problemdetails_mapper::ProblemDetailsMapper, traits::into_problemdetails::IntoProblemDetails};

#[derive(Debug)]
struct TestError;

impl TestError {
    pub fn new() -> Self {
        Self
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl error::Error for TestError {}

impl IntoProblemDetails for TestError {
    fn into_problemdetails(&self) -> problem_details::ProblemDetails {
        ProblemDetailsBuilder::build_bad_request(
            Some("test"),
            None::<String>)
    }
}

#[test]
fn mapping_without_default_should_return_none() {
    let setup_result = ProblemDetailsMapper::setup(|_| {});

    assert_eq!(setup_result.is_ok(), true);

    let error = MapperError::new("test", None);
    let result = ProblemDetailsMapper::map(Box::new(error));

    assert_eq!(result.is_none(), true);
}

#[test]
fn mapping_with_own_error_should_function_correctly() {    
    let setup_result = ProblemDetailsMapper::setup(|options| {
        options.map::<TestError>();
    });

    assert_eq!(setup_result.is_ok(), true);

    let error = TestError::new();
    let result = ProblemDetailsMapper::map(Box::new(error));

    assert_eq!(result.is_some(), true);

    let detail = result.unwrap();

    assert_eq!(detail.status, Some(StatusCode::BAD_REQUEST));
    assert_eq!(detail.detail, Some("test".to_string()));
}

#[test]
fn mapping_with_default_should_return_default() {    
    let setup_result = ProblemDetailsMapper::setup(|options| {
        options.map_std_err();
    });

    assert_eq!(setup_result.is_ok(), true);

    let error = TestError::new();
    let result = ProblemDetailsMapper::map(Box::new(error));

    assert_eq!(result.is_some(), true);

    let detail = result.unwrap();

    assert_eq!(detail.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
    assert_eq!(detail.detail, Some("".to_string()));
    assert_eq!(detail.r#type.unwrap().to_string(), "https://errors.io/unkownerror".to_string())
}