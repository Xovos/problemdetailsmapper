use std::{error::Error, sync::RwLock};
use problem_details::ProblemDetails;
use crate::{traits::into_problemdetails::IntoProblemDetails, errors::mapper_error::MapperError};

type MapFn = &'static (dyn Fn(&Box<dyn Error>) -> Option<ProblemDetails> + Sync);

static MAPS: RwLock<Vec<MapFn>> = RwLock::new(Vec::new());

pub struct ProblemDetailsMapper;

impl ProblemDetailsMapper {
    pub fn map(error: Box<dyn Error>) -> Option<ProblemDetails> {
        let mut details: Option<ProblemDetails> = None;

        if let Ok(maps) = MAPS.read() {
            for map in maps.iter() {
                details = details.or_else(|| map(&error));
            }
        }

        details
    }

    pub fn setup<F>(setup: F) -> Result<(), MapperError> 
        where F: Fn(&mut ProblemDetailsOptions) {
        let mut maps = MAPS.write().map_err(|e| MapperError::new("error getting write lock", Some(Box::new(e))))?;
        let mut options = ProblemDetailsOptions::new();
        setup(&mut options);
        *maps = options.maps;
        Ok(())
    }
}

pub struct ProblemDetailsOptions {
    maps: Vec<MapFn>
}

impl ProblemDetailsOptions {
    pub fn new() -> Self {
        Self {
            maps: Vec::new()
        }
    }

    pub fn map<TType>(&mut self)
        where TType : Error + IntoProblemDetails + 'static {
        self.maps.push(&Self::map_type::<TType>)
    }

    pub fn map_std_err(&mut self) {
        self.maps.push(&Self::map_default_error)
    }

    fn map_type<TType>(error: &Box<dyn Error>) -> Option<ProblemDetails>
        where TType : Error + IntoProblemDetails + 'static {
        let concrete_error = error.as_ref().downcast_ref::<TType>()?;
        Some(concrete_error.into_problemdetails())
    }

    fn map_default_error(error: &Box<dyn Error>) -> Option<ProblemDetails> {
        use crate::builders::problemdetails_builder::ProblemDetailsBuilder;
        const TYPE: &'static str = "https://errors.io/unkownerror";
        Some(ProblemDetailsBuilder::build_server_error(
            Some(format!("{}", error)),
            Some(TYPE)))
    }
}