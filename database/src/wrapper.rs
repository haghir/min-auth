use futures_util::StreamExt;
use mysql_async::{
    prelude::*,
    BinaryProtocol, Conn, Params, QueryResult, Result, ResultSetStream,
};
use std::marker::{Unpin, Send};

pub struct ResultWrapper<'a, 't, T, P> {
    result: QueryResult<'a, 't, P>,
    _dummy: Option<T>,
}

pub struct StreamWrapper<'r, 'a, 't, T, P> {
    stream: ResultSetStream<'r, 'a, 't, T, P>
}

impl<'a, 't, T, P> ResultWrapper<'a, 't, T, P> {
    pub fn new(result: QueryResult<'a, 't, P>) -> Self
    where
        P: Unpin + Protocol,
    {
        ResultWrapper::<T, P> { result, _dummy: None }
    }

    pub async fn stream<'r>(&'r mut self) ->
        Result<Option<StreamWrapper<'r, 'a, 't, T, P>>>
    where
        T: Unpin + FromRow + Send + 'static,
        P: Unpin + Protocol,
    {
        if let Some(stream) = self.result.stream::<T>().await? {
            Ok(Some(StreamWrapper::new(stream)))
        } else {
            Ok(None)
        }
    }
}

impl<'r, 'a, 't, T, P> StreamWrapper<'r, 'a, 't, T, P> {
    pub fn new(stream: ResultSetStream<'r, 'a, 't, T, P>) -> Self
    where
        T: Unpin + FromRow + Send + 'static,
        P: Unpin + Protocol,
    {
        StreamWrapper { stream }
    }

    pub async fn next(&mut self) -> Option<Result<T>>
    where
        T: Unpin + FromRow + Send + 'static,
        P: Unpin + Protocol,
    {
        self.stream.next().await
    }
}

pub trait QueryWrapper<T> {
    fn sql(&self) -> String;
    fn params(&self) -> Params;

    async fn select<'a>(&self, conn: &'a mut Conn) ->
        Result<ResultWrapper<'a, 'static, T, BinaryProtocol>>
    where
        T: Unpin + FromRow + Send + 'static
    {
        let sql = self.sql();
        let params = self.params();
        let query = sql.with(params);
        Ok(ResultWrapper::new(query.run(conn).await?))
    }
}
