use bgpkit_parser::models::*;
use bgpkit_parser::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;

#[pymodule]
fn pybgpkit_parser(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    fn convert_elem(elem: BgpElem) -> Elem {
        Elem {
            timestamp: elem.timestamp,
            elem_type: match elem.elem_type {
                ElemType::ANNOUNCE => "A".to_string(),
                ElemType::WITHDRAW => "W".to_string(),
            },
            peer_ip: elem.peer_ip.to_string(),
            peer_asn: elem.peer_asn.to_u32(),
            prefix: elem.prefix.to_string(),
            next_hop: elem.next_hop.map(|v| v.to_string()),
            as_path: elem.as_path.map(|v| v.to_string()),
            origin_asns: elem
                .origin_asns
                .map(|v| v.into_iter().map(|x| x.to_u32()).collect()),
            origin: elem.origin.map(|v| v.to_string()),
            local_pref: elem.local_pref,
            med: elem.med,
            communities: elem
                .communities
                .map(|v| v.into_iter().map(|x| x.to_string()).collect()),
            atomic: match elem.atomic {
                true => Some("AG".to_string()),
                false => Some("NAG".to_string()),
            },
            aggr_asn: elem.aggr_asn.map(|v| v.to_u32()),
            aggr_ip: elem.aggr_ip.map(|v| v.to_string()),
        }
    }

    #[pyclass]
    #[derive(Clone, PartialEq, Serialize)]
    pub struct Elem {
        #[pyo3(get, set)]
        pub timestamp: f64,
        #[pyo3(get, set)]
        pub elem_type: String,
        #[pyo3(get, set)]
        pub peer_ip: String,
        #[pyo3(get, set)]
        pub peer_asn: u32,
        #[pyo3(get, set)]
        pub prefix: String,
        #[pyo3(get, set)]
        pub next_hop: Option<String>,
        #[pyo3(get, set)]
        pub as_path: Option<String>,
        #[pyo3(get, set)]
        pub origin_asns: Option<Vec<u32>>,
        #[pyo3(get, set)]
        pub origin: Option<String>,
        #[pyo3(get, set)]
        pub local_pref: Option<u32>,
        #[pyo3(get, set)]
        pub med: Option<u32>,
        #[pyo3(get, set)]
        pub communities: Option<Vec<String>>,
        #[pyo3(get, set)]
        pub atomic: Option<String>,
        #[pyo3(get, set)]
        pub aggr_asn: Option<u32>,
        #[pyo3(get, set)]
        pub aggr_ip: Option<String>,
    }

    #[pymethods]
    impl Elem {
        pub fn to_dict(&self, py: Python) -> PyObject {
            use pyo3::types::PyDict;
            let dict = PyDict::new(py);
            dict.set_item("timestamp", self.timestamp).unwrap();
            dict.set_item("elem_type", self.elem_type.clone()).unwrap();
            dict.set_item("peer_ip", self.peer_ip.clone()).unwrap();
            dict.set_item("peer_asn", self.peer_asn).unwrap();
            dict.set_item("prefix", self.prefix.clone()).unwrap();
            dict.set_item("next_hop", self.next_hop.clone()).unwrap();
            dict.set_item("as_path", self.as_path.clone()).unwrap();
            dict.set_item("origin_asns", self.origin_asns.clone())
                .unwrap();
            dict.set_item("origin", self.origin.clone()).unwrap();
            dict.set_item("local_pref", self.local_pref).unwrap();
            dict.set_item("med", self.med).unwrap();
            dict.set_item("communities", self.communities.clone())
                .unwrap();
            dict.set_item("atomic", self.atomic.clone()).unwrap();
            dict.set_item("aggr_asn", self.aggr_asn).unwrap();
            dict.set_item("aggr_ip", self.aggr_ip.clone()).unwrap();
            dict.into()
        }

        fn __getstate__(&self, py: Python) -> PyObject {
            self.to_dict(py)
        }

        #[pyo3(name = "__str__")]
        fn str_repr(&self) -> PyResult<String> {
            Ok(serde_json::to_string(self).unwrap().to_string())
        }
    }

    #[pyclass]
    struct Parser {
        elem_iter: ElemIterator<Box<dyn Send + Read>>,
    }

    unsafe impl Send for Parser {}
    unsafe impl Sync for Parser {}

    #[pymethods]
    impl Parser {
        #[new]
        #[pyo3(signature = (url, filters=None, cache_dir=None))]
        fn new(
            url: String,
            filters: Option<HashMap<String, String>>,
            cache_dir: Option<String>,
        ) -> PyResult<Self> {
            let mut parser = match cache_dir {
                None => BgpkitParser::new(url.as_str()).unwrap(),
                Some(dir) => BgpkitParser::new_cached(url.as_str(), dir.as_str()).unwrap(),
            };

            if let Some(filters) = filters {
                for (k, v) in filters {
                    parser = match parser.add_filter(k.as_str(), v.as_str()) {
                        Ok(p) => p,
                        Err(e) => return Err(PyValueError::new_err(e.to_string())),
                    }
                }
            }
            let elem_iter = parser.into_iter();
            Ok(Parser { elem_iter })
        }

        fn parse_all(&mut self, py: Python) -> PyResult<Vec<Py<Elem>>> {
            let mut elems = vec![];
            for e in self.elem_iter.by_ref() {
                elems.push(Py::new(py, convert_elem(e))?);
            }
            Ok(elems)
        }

        fn parse_next(&mut self, py: Python) -> PyResult<Option<Py<Elem>>> {
            Ok(self
                .elem_iter
                .next()
                .map(|e| Py::new(py, convert_elem(e)).unwrap()))
        }

        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> Option<Py<Elem>> {
            slf.elem_iter
                .next()
                .map(|e| Py::new(py, convert_elem(e)).unwrap())
        }
    }

    m.add_class::<Elem>()?;
    m.add_class::<Parser>()?;
    Ok(())
}
