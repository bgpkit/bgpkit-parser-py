use std::collections::HashMap;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use bgpkit_parser::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

macro_rules! option_to_string{
    ($a:expr) => {
        if let Some(v) = $a {
            v.to_string()
        } else {
            String::new()
        }
    }
}

#[inline(always)]
pub fn option_to_string_vec(o: &Option<Vec<String>>) -> String {
    if let Some(v) = o {
        v.iter()
            .join(" ")
    } else {
        String::new()
    }
}
#[pymodule]
fn pybgpkit_parser(_py: Python, m: &PyModule) -> PyResult<()> {

    impl Display for Elem {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let format = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                self.elem_type, &self.timestamp,
                &self.peer_ip,
                &self.peer_asn,
                &self.prefix,
                option_to_string!(&self.as_path),
                option_to_string!(&self.origin),
                option_to_string!(&self.next_hop),
                option_to_string!(&self.local_pref),
                option_to_string!(&self.med),
                option_to_string_vec(&self.communities),
                option_to_string!(&self.atomic),
                option_to_string!(&self.aggr_asn),
                option_to_string!(&self.aggr_ip),
            );
            write!(f, "{}", format)
        }
    }

    fn convert_elem(elem: BgpElem) -> Elem {
        Elem {
            timestamp: elem.timestamp,
            elem_type: match elem.elem_type {
                ElemType::ANNOUNCE => {"A".to_string()}
                ElemType::WITHDRAW => {"W".to_string()}
            }
            ,
            peer_ip: elem.peer_ip.to_string(),
            peer_asn: elem.peer_asn.asn,
            prefix: elem.prefix.to_string(),
            next_hop: match elem.next_hop {
                None => {None}
                Some(v) => {Some(v.to_string())}
            },
            as_path: match elem.as_path {
                None => {None}
                Some(v) => {Some(v.to_string())}
            },
            origin_asns: match elem.origin_asns {
                None => {None}
                Some(v) => {Some(v.into_iter().map(|x|x.asn).collect())}
            },
            origin: match elem.origin {
                None => {None}
                Some(v) => {Some(v.to_string())}
            },
            local_pref: elem.local_pref,
            med: elem.med,
            communities: match elem.communities {
                None => {None}
                Some(v) => {Some(v.into_iter().map(|x|x.to_string()).collect())}
            },
            atomic: match elem.atomic {
                None => {Some("NAG".to_string())}
                Some(v) => {match v {
                    AtomicAggregate::NAG => {Some("NAG".to_string())}
                    AtomicAggregate::AG => {Some("AG".to_string())}
                }}
            },
            aggr_asn: match elem.aggr_asn {
                None => {None}
                Some(v) => {Some(v.asn)}
            },
            aggr_ip: match elem.aggr_ip {
                None => {None}
                Some(v) => {Some(v.to_string())}
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Elem {
        #[pyo3(get,set)]
        pub timestamp: f64,
        #[pyo3(get,set)]
        pub elem_type: String,
        #[pyo3(get,set)]
        pub peer_ip: String,
        #[pyo3(get,set)]
        pub peer_asn: u32,
        #[pyo3(get,set)]
        pub prefix: String,
        #[pyo3(get,set)]
        pub next_hop: Option<String>,
        #[pyo3(get,set)]
        pub as_path: Option<String>,
        #[pyo3(get,set)]
        pub origin_asns: Option<Vec<u32>>,
        #[pyo3(get,set)]
        pub origin: Option<String>,
        #[pyo3(get,set)]
        pub local_pref: Option<u32>,
        #[pyo3(get,set)]
        pub med: Option<u32>,
        #[pyo3(get,set)]
        pub communities: Option<Vec<String>>,
        #[pyo3(get,set)]
        pub atomic: Option<String>,
        #[pyo3(get,set)]
        pub aggr_asn: Option<u32>,
        #[pyo3(get,set)]
        pub aggr_ip: Option<String>
    }

    #[pymethods]
    impl Elem {
        fn __str__(&self) -> PyResult<String> {
            Ok(self.to_string())
        }
    }

    #[pyclass]
    #[pyo3(text_signature = "(url, filters, /)")]
    struct Parser {
        elem_iter: ElemIterator,
    }

    #[pymethods]
    impl Parser {
        #[new]
        fn new(url: String, filters: Option<HashMap<String, String>>) -> PyResult<Self> {
            let mut parser = BgpkitParser::new(url.as_str()).unwrap();

            if let Some(filters) = filters {
                for (k,v) in filters {
                   parser = match parser.add_filter(k.as_str(), v.as_str()) {
                       Ok(p) => {p}
                       Err(e) => {
                           return Err(PyValueError::new_err(e.to_string()))
                       }
                   }
                }
            }
            let elem_iter = parser.into_iter();
            Ok(Parser{elem_iter})
        }

        fn parse_all(&mut self) -> PyResult<Vec<Elem>> {
            let mut elems = vec![];

            loop {
                match self.elem_iter.next() {
                    None => {break}
                    Some(e) => {
                        elems.push(convert_elem(e))
                    }
                }
            }
            Ok(elems)
        }

        fn parse_next(&mut self) -> PyResult<Option<Elem>> {
            Ok(match self.elem_iter.next() {
                None => {None}
                Some(e) => {Some(convert_elem(e))}
            })
        }

        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>) -> Option<Elem> {
            let e = slf.elem_iter.next();
            match e {
                None => {None}
                Some(e) => {Some(convert_elem(e))}
            }
        }
    }

    m.add_class::<Parser>()?;
    m.add_class::<Elem>()?;
    Ok(())
}