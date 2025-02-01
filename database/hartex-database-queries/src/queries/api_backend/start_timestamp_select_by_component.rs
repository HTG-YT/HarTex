// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

use std::env;
use itertools::Itertools;
use tokio::net::TcpStream;
use wtx::database::Executor as _;
use wtx::database::Records;
use wtx::database::client::postgres::Executor;
use wtx::database::client::postgres::ExecutorBuffer;
use wtx::misc::Uri;
use crate::result::IntoCrateResult;
pub struct StartTimestampSelectByComponent {
    db_executor: Option<Executor<wtx::Error, ExecutorBuffer, TcpStream>>,
    executor_constructor: for<'a> fn(Uri<&'a str>) -> crate::internal::Ret<'a>,
    component: String,
}
impl StartTimestampSelectByComponent {
    #[must_use = "Queries must be executed after construction"]
    pub fn bind(component: String) -> Self {
        Self {
            db_executor: None,
            executor_constructor: crate::internal::__internal_executor_constructor
                as for<'a> fn(Uri<&'a str>) -> crate::internal::Ret<'a>,
            component,
        }
    }
    pub async fn executor(mut self) -> crate::result::Result<Self> {
        self.db_executor
            .replace(
                (self
                    .executor_constructor)(
                        Uri::new(&env::var("API_BACKEND_PGSQL_URL").unwrap()),
                    )
                    .await?,
            );
        Ok(self)
    }
    pub async fn one(
        self,
    ) -> crate::result::Result<crate::tables::api_backend::StartTimestamps> {
        self.db_executor
            .ok_or(
                crate::result::Error::Generic(
                    ".executor() has not been called on this query yet",
                ),
            )?
            .fetch_with_stmt(
                "SELECT * FROM \"APIBackend\".public.\"StartTimestamps\" WHERE \"component\" = $1",
                (self.component,),
            )
            .await
            .into_crate_result()
            .map(|record| crate::tables::api_backend::StartTimestamps::try_from(record))
            .flatten()
    }
}
