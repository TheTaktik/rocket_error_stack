use rocket::http::Status;
use rocket::log::private::warn;
use rocket::response::{self, Responder};
use rocket::{Request, Response};
use rocket::yansi::Paint;
use std::fmt::Display;

pub trait StatusCodeReport {
    fn status(&self) -> Status {
        Status::InternalServerError
    }
}

#[derive(Debug)]
pub struct Report<E>(pub error_stack::Report<E>);

impl<R, E> From<R> for crate::Report<E>
where
    R: Into<error_stack::Report<E>>
{
    fn from(report: R) -> Self {
        Report(report.into())
    }
}

impl<'r, E> Responder<'r, 'static> for Report<E>
    where E: 'static + StatusCodeReport + Display + Send + Sync
{
    fn respond_to(self, _request: &Request<'_>) -> response::Result<'static> {
        let status = self.0.current_context().status();
        warn!("Debug: {:?}", Paint::default(self.0));
        warn!("Returning {}", status);
        Response::build().status(status).ok()
    }
}

pub type Result<T, C> = std::result::Result<T, Report<C>>;
