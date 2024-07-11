use eval::{Eval, EvalResult, EvalStatus};
use utoipa::OpenApi;

use crate::routes::{cleanup, containers, eval, languages};

#[derive(OpenApi)]
#[openapi(
    paths(cleanup::cleanup, containers::containers, eval::eval, languages::languages),
    components(schemas(Eval, EvalResult, EvalStatus))
)]
pub struct Docs;
