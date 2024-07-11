use utoipa::OpenApi;

use crate::routes::Routes;

#[derive(OpenApi)]
#[openapi(
	nest(
		(path = "/api", api = Routes)
	)
)]
pub struct Docs;
