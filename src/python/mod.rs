#![allow(non_local_definitions)] // pyo3-macros 0.20 generates impls inside a trampoline

use crate::api::{DDApi, DDnetClient};
use crate::api::ddnet::DDnetApi;
use crate::error::{Error, Result};
use crate::scheme::ddnet::prelude::MasterServer;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::Serialize;
use std::time::Duration;

create_exception!("ddapi", DDError, PyException, "An error raised by the ddapi library.");

fn err_to_pyerr(err: Error) -> PyErr {
    DDError::new_err(format!("{err}"))
}

/// Converts a `serde_json::Value` into a plain Python object
/// (`None`, `bool`, `int`, `float`, `str`, `list`, `dict`).
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else {
                Ok(n.as_f64().unwrap().into_py(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(elements) => {
            let list = PyList::empty(py);
            for element in elements {
                list.append(json_to_py(py, element)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (key, value) in map {
                dict.set_item(key, json_to_py(py, value)?)?;
            }
            Ok(dict.into())
        }
    }
}

/// Converts a `Result<T>` into a Python object, re-acquiring the GIL.
/// Meant to be called from inside a tokio future.
pub fn convert<T>(result: Result<T>) -> PyResult<PyObject>
where
    T: Serialize + Send,
{
    match result {
        Ok(value) => Python::with_gil(|py| {
            let json = serde_json::to_value(&value)
                .map_err(|e| DDError::new_err(format!("{e}")))?;
            json_to_py(py, &json)
        }),
        Err(err) => Err(err_to_pyerr(err)),
    }
}

fn master_server_from_index(index: i64) -> Option<MasterServer> {
    match index {
        1 => Some(MasterServer::One),
        2 => Some(MasterServer::Two),
        3 => Some(MasterServer::Three),
        4 => Some(MasterServer::Four),
        _ => None,
    }
}

fn custom_master_into_py<'py>(py: Python<'py>, inner: DDApi, master: i64) -> PyResult<PyObject> {
    match master_server_from_index(master) {
        Some(server) => pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.custom_master(server).await)
        })
        .map(|b| b.into()),
        None => Err(DDError::new_err(format!(
            "master server index must be between 1 and 4, got {master}"
        ))),
    }
}

fn custom_master_into_py_client<'py>(
    py: Python<'py>,
    inner: DDnetClient,
    master: i64,
) -> PyResult<PyObject> {
    match master_server_from_index(master) {
        Some(server) => pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.custom_master(server).await)
        })
        .map(|b| b.into()),
        None => Err(DDError::new_err(format!(
            "master server index must be between 1 and 4, got {master}"
        ))),
    }
}

#[pyclass(module = "ddapi", name = "DDApi")]
pub struct DDApiPy {
    inner: DDApi,
}

#[pymethods]
impl DDApiPy {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> Self {
        Self {
            inner: DDApi::new(),
        }
    }

    #[cfg(feature = "cache")]
    #[pyo3(text_signature = "(capacity: int, ttl_seconds: int)")]
    fn set_cache(&mut self, capacity: i64, ttl_seconds: i64) {
        self.inner.set_cache(capacity as u64, Duration::from_secs(ttl_seconds.try_into().unwrap()));
    }

    #[pyo3(text_signature = "()")]
    fn master(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.master().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(master: int)")]
    fn custom_master(&self, py: Python, master: i64) -> PyResult<PyObject> {
        custom_master_into_py(py, self.inner.clone(), master)
    }

    #[pyo3(text_signature = "()")]
    fn skins(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.skins().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn player(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.player(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn query(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(map: str)")]
    fn query_map(&self, py: Python, map: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query_map(&map).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn query_mapper(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query_mapper(&player).await)
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
    fn releases_map(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.releases_map().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "()")]
    fn status(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.status().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "()")]
    fn latest_finish(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.latest_finish().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(latest: int)")]
    fn latest_finish_with_latest(&self, py: Python, latest: i64) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.latest_finish_with_latest(latest as usize).await)
        })
        .map(|b| b.into())
    }
}

#[pyclass(module = "ddapi", name = "DDnetClient")]
pub struct DDnetClientPy {
    inner: DDnetClient,
}

#[pymethods]
impl DDnetClientPy {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> Self {
        Self {
            inner: DDnetClient::new(),
        }
    }

    #[cfg(feature = "cache")]
    #[pyo3(text_signature = "(capacity: int, ttl_seconds: int)")]
    fn set_cache(&mut self, capacity: i64, ttl_seconds: i64) {
        self.inner.set_cache(capacity as u64, Duration::from_secs(ttl_seconds.try_into().unwrap()));
    }

    #[pyo3(text_signature = "()")]
    fn master(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.master().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(master: int)")]
    fn custom_master(&self, py: Python, master: i64) -> PyResult<PyObject> {
        custom_master_into_py_client(py, self.inner.clone(), master)
    }

    #[pyo3(text_signature = "()")]
    fn skins(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.skins().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn player(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.player(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn query(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query(&player).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(map: str)")]
    fn query_map(&self, py: Python, map: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query_map(&map).await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(player: str)")]
    fn query_mapper(&self, py: Python, player: String) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.query_mapper(&player).await)
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
    fn releases_map(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.releases_map().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "()")]
    fn status(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.status().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "()")]
    fn latest_finish(&self, py: Python) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.latest_finish().await)
        })
        .map(|b| b.into())
    }

    #[pyo3(text_signature = "(latest: int)")]
    fn latest_finish_with_latest(&self, py: Python, latest: i64) -> PyResult<PyObject> {
        let inner = self.inner.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            convert(inner.latest_finish_with_latest(latest as usize).await)
        })
        .map(|b| b.into())
    }
}

#[cfg(feature = "ddstats")]
mod ddstats;

#[pymodule]
fn _ddapi(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DDApiPy>()?;
    m.add_class::<DDnetClientPy>()?;
    #[cfg(feature = "ddstats")]
    m.add_class::<ddstats::DDstatsClientPy>()?;
    m.add("DDError", py.get_type::<DDError>())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "Async Python bindings for the DDNet and DDStats APIs.")?;
    Ok(())
}