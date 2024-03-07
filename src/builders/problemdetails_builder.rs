use http::{StatusCode, Uri};
use problem_details::ProblemDetails;

pub struct ProblemDetailsBuilder;

impl ProblemDetailsBuilder {
    const DEFAULT_NOT_FOUND_DETAIL: &'static str = "The requested resource could not be found on the server.";
    const DEFAULT_BAD_REQUEST_DETAIL: &'static str = "The request is invalid or malformed.";
    const DEFAULT_SERVER_ERROR_DETAIL: &'static str = "An unexpected error occurred on the server.";

    const DEFAULT_NOT_FOUND_TITLE: &'static str = "Resource not found";
    const DEFAULT_BAD_REQUEST_TITLE: &'static str = "Bad Request";
    const DEFAULT_SERVER_ERROR_TITLE: &'static str = "Internal Server Error";


    pub fn build_not_found(detail: Option<impl Into<String>>, rtype: Option<impl Into<String>>) -> ProblemDetails {
        let detail = match detail {
            Some(d) => d.into(),
            None => Self::DEFAULT_NOT_FOUND_DETAIL.to_string()
        };


        ProblemDetailsBuilder::build(
            StatusCode::NOT_FOUND,
            Self::DEFAULT_NOT_FOUND_TITLE.to_string(),
            detail,
            rtype,
            None::<String>)
    }

    pub fn build_bad_request(detail: Option<impl Into<String>>, rtype: Option<impl Into<String>>) -> ProblemDetails {
        let detail = match detail {
            Some(d) => d.into(),
            None => Self::DEFAULT_BAD_REQUEST_DETAIL.to_string()
        };


        ProblemDetailsBuilder::build(
            StatusCode::BAD_REQUEST,
            Self::DEFAULT_BAD_REQUEST_TITLE.to_string(),
            detail,
            rtype,
            None::<String>)
    }

    pub fn build_server_error(detail: Option<impl Into<String>>, rtype: Option<impl Into<String>>) -> ProblemDetails {
        let detail = match detail {
            Some(d) => d.into(),
            None => Self::DEFAULT_SERVER_ERROR_DETAIL.to_string()
        };


        ProblemDetailsBuilder::build(
            StatusCode::INTERNAL_SERVER_ERROR,
            Self::DEFAULT_SERVER_ERROR_TITLE.to_string(),
            detail,
            rtype,
            None::<String>)
    }

    pub fn build(status: StatusCode, title: String, detail: String, rtype: Option<impl Into<String>>, instance: Option<impl Into<String>>) -> ProblemDetails {
        let mut builder = ProblemDetails::new()
            .with_status(status)
            .with_title(title)
            .with_detail(detail);

        if let Some(t) = rtype {
            builder = builder.with_type(t.into().parse::<Uri>().unwrap());
        }

        if let Some(i) = instance {
            builder = builder.with_instance(i.into().parse::<Uri>().unwrap());
        }

        return builder;
    }
}