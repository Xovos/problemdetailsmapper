use problem_details::ProblemDetails;

pub trait IntoProblemDetails {
    fn into_problemdetails(&self) -> ProblemDetails;
}