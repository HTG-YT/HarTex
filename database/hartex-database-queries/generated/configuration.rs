// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod plugin_enabled {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct PluginEnabledParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub plugin: T1,
            pub guild_id: T2,
        }
        pub struct BoolQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> bool,
            mapper: fn(bool) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> BoolQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(bool) -> R) -> BoolQuery<'a, C, R, N> {
                BoolQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn plugin_enabled() -> PluginEnabledStmt {
            PluginEnabledStmt(cornucopia_async::private::Stmt::new(
                "SELECT EXISTS(
    SELECT
        TRUE
    FROM
        \"Nightly\".\"GuildConfigurations\"
    WHERE
        \"enabled_plugins\" @> array[ $1 ] AND
        \"guild_id\" = $2
)",
            ))
        }
        pub struct PluginEnabledStmt(cornucopia_async::private::Stmt);
        impl PluginEnabledStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                plugin: &'a T1,
                guild_id: &'a T2,
            ) -> BoolQuery<'a, C, bool, 2> {
                BoolQuery {
                    client,
                    params: [plugin, guild_id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql>
            cornucopia_async::Params<'a, PluginEnabledParams<T1, T2>, BoolQuery<'a, C, bool, 2>, C>
            for PluginEnabledStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a PluginEnabledParams<T1, T2>,
            ) -> BoolQuery<'a, C, bool, 2> {
                self.bind(client, &params.plugin, &params.guild_id)
            }
        }
    }
}
