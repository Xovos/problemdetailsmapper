use std::{error::Error, sync::RwLock};

use http::StatusCode;
use problem_details::ProblemDetails;

use crate::{into_problemdetails::IntoProblemDetails, mapper_error::MapperError};

type MapFn = &'static (dyn Fn(&Box<dyn Error>) -> Option<ProblemDetails> + Sync);


static MAPS: RwLock<Vec<MapFn>> = RwLock::new(Vec::new());

pub struct ProblemDetailsMapper;

impl ProblemDetailsMapper {
    pub fn map(error: Box<dyn Error>) -> Result<(StatusCode, ProblemDetails), MapperError> {
        let details = Self::map_types(error)?;
        let status = details.status
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        return Ok(( status, details ));
    }

    pub fn setup<F>(setup: F) -> Result<(), MapperError> 
        where F: Fn(&mut ProblemDetailsOptions) {
        let mut maps = MAPS.write().map_err(|e| MapperError::new("error getting write lock", Some(Box::new(e))))?;
        let mut options = ProblemDetailsOptions::new();
        setup(&mut options);
        *maps = options.maps;
        Ok(())
    }

    fn map_types(error: Box<dyn Error>) -> Result<ProblemDetails, MapperError> {
        let mut details: Option<ProblemDetails> = None;

        for map in MAPS.read().map_err(|e| MapperError::new("register before mapping.", Some(Box::new(e))))?.iter() {
            details = details.or_else(|| map(&error));
        }

        Ok(details
            .unwrap_or_else(|| error.into_problemdetails()))
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

    fn map_type<TType>(error: &Box<dyn Error>) -> Option<ProblemDetails>
        where TType : Error + IntoProblemDetails + 'static {
        let concrete_error = error.as_ref().downcast_ref::<TType>()?;
        Some(concrete_error.into_problemdetails())
    }
}