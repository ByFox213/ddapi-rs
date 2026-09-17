#![allow(non_local_definitions)] // pyo3-macros 0.20 generates impls inside a trampoline

use crate::api::DDstatsClient;
use crate::api::ddstats::DDstats;
use crate::python::convert;
use pyo3::prelude::*;
use std::time::Duration;

#[pyclass(module = "ddapi", name = "DDstatsClient")]
pub struct DDstatsClientPy {
    inner: DDstatsClient,
}

#[pymethods]
impl DDstatsClientPy {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> Self {
        Self {
            inner: DDstatsClient::new(),
        }
    }

    #[cfg(feature = "cache")]
    #[pyo3(text_signature = "(capacity: int, ttl_seconds: int)")]
    fn set_cache(&mut self, capacity: i64, ttl_seconds: i64) {
        self.inner.set_cache(capacity as u64, Duration::from_secs(ttl_seconds.try_into().unwrap()));
    }

    #[pyo3(text_signature = "(player: str)")]
    fn player(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.player(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(map: str)")]
    fn map(&self, py: Python, map: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.map(&map).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "()")]
    fn maps(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.maps().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn profile(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.profile(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn teero(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.teero(&player).await)
        })
        .map(|b| b.into())
    }
}