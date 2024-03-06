use problem_details::ProblemDetails;

use super::problemdetails_builder::ProblemDetailsBuilder;

pub trait IntoProblemDetails {
    fn into_problemdetails(&self) -> ProblemDetails;
}

impl IntoProblemDetails for Box<dyn std::error::Error> {
    fn into_problemdetails(&self) -> ProblemDetails {
        ProblemDetailsBuilder::build_server_error(
            Some(format!("{}", self)),
            None::<String>)
    }
}