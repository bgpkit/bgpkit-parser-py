use bgpkit_parser::models::*;
use bgpkit_parser::*;
use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::io::Read;

#[pymodule]
fn pybgpkit_parser(_py: Python, m: &PyModule) -> PyResult<()> {
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
                true => {Some("AG".to_string())}
                false => {Some("NAG".to_string())}
            },
            aggr_asn: elem.aggr_asn.map(|v| v.to_u32()),
            aggr_ip: elem.aggr_ip.map(|v| v.to_string()),
        }
    }

    #[derive(Clone, PartialEq, FromPyObject, IntoPyObject)]
    pub struct Elem {
        pub timestamp: f64,
        pub elem_type: String,
        pub peer_ip: String,
        pub peer_asn: u32,
        pub prefix: String,
        pub next_hop: Option<String>,
        pub as_path: Option<String>,
        pub origin_asns: Option<Vec<u32>>,
        pub origin: Option<String>,
        pub local_pref: Option<u32>,
        pub med: Option<u32>,
        pub communities: Option<Vec<String>>,
        pub atomic: Option<String>,
        pub aggr_asn: Option<u32>,
        pub aggr_ip: Option<String>,
    }

    #[pyclass]
    struct Parser {
        elem_iter: ElemIterator<Box<dyn Send + Read>>,
    }

    #[pymethods]
    impl Parser {
        #[new]
        #[pyo3(text_signature = "(url, filters, /)")]
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

        fn parse_all(&mut self) -> PyResult<Vec<Elem>> {
            let mut elems = vec![];

            loop {
                match self.elem_iter.next() {
                    None => break,
                    Some(e) => elems.push(convert_elem(e)),
                }
            }
            Ok(elems)
        }

        fn parse_next(&mut self) -> PyResult<Option<Elem>> {
            Ok(self.elem_iter.next().map(convert_elem))
        }

        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>) -> Option<Elem> {
            let e = slf.elem_iter.next();
            e.map(convert_elem)
        }
    }

    m.add_class::<Parser>()?;
    Ok(())
}
