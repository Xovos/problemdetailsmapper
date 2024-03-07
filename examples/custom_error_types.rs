use problem_details_mapper::{
    IntoProblemDetails,
    ProblemDetails,
    ProblemDetailsBuilder,
    ProblemDetailsMapper
};

#[derive(Debug)]
struct TestError;

impl TestError {
    pub fn new() -> Self {
        Self
    }
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for TestError {}

impl IntoProblemDetails for TestError {
    fn into_problemdetails(&self) -> ProblemDetails {
        ProblemDetailsBuilder::build_bad_request(
            Some("test"),
            None::<String>)
    }
}

fn main() {
    // Setup the mapper settings
    ProblemDetailsMapper::setup(|options| {
        // will register the test error its own mapping
        // provided by IntoProblemDetails.
        options.map::<TestError>();
    }).unwrap();

    // Map an error to a Problem Details response
    let error = TestError::new();
    let problem_details = ProblemDetailsMapper::map(Box::new(error));

    // Output the Problem Details response
    println!("{:?}", problem_details);
}