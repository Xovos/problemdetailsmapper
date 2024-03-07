pub mod problemdetails_mapper;
pub mod into_problemdetails;
pub mod problemdetails_builder;
pub mod mapper_error;

#[cfg(test)]
mod tests {
    use std::error;

    use http::StatusCode;

    use crate::{into_problemdetails::IntoProblemDetails, problemdetails_builder::ProblemDetailsBuilder, problemdetails_mapper::ProblemDetailsMapper};

    use self::mapper_error::MapperError;

    use super::*;

    #[test]
    fn mapping_should_function_correctly() {
        let setup_result = ProblemDetailsMapper::setup(|_| {});

        assert_eq!(setup_result.is_ok(), true);

        let error = MapperError::new("test", None);
        let result = ProblemDetailsMapper::map(Box::new(error));

        assert_eq!(result.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
        assert_eq!(result.detail, Some("test".to_string()));
    }

    #[test]
    fn mapping_with_own_error_should_function_correctly() {
        use std::fmt;

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

        let setup_result = ProblemDetailsMapper::setup(|options| {
            options.map::<TestError>();
        });

        dbg!(&setup_result);

        assert_eq!(setup_result.is_ok(), true);

        let error = TestError::new();
        let result = ProblemDetailsMapper::map(Box::new(error));

        assert_eq!(result.status, Some(StatusCode::BAD_REQUEST));
        assert_eq!(result.detail, Some("test".to_string()));
    }
}
