use bgpkit_parser::models::*;
use bgpkit_parser::parser::Filter as BgpkitFilter;
use bgpkit_parser::*;
use pyo3::conversion::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
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
            peer_bgp_id: elem.peer_bgp_id.map(|v| v.to_string()),
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
            only_to_customer: elem.only_to_customer.map(|v| v.to_u32()),
        }
    }

    fn convert_route_elem(elem: BgpRouteElem) -> RouteElem {
        RouteElem {
            timestamp: elem.timestamp,
            elem_type: match elem.elem_type {
                ElemType::ANNOUNCE => "A".to_string(),
                ElemType::WITHDRAW => "W".to_string(),
            },
            peer_ip: elem.peer_ip.to_string(),
            peer_asn: elem.peer_asn.to_u32(),
            prefix: elem.prefix.to_string(),
            as_path: elem.as_path.map(|v| v.to_string()),
        }
    }

    fn new_parser(
        url: &str,
        cache_dir: Option<&str>,
    ) -> PyResult<BgpkitParser<Box<dyn Send + Read>>> {
        match cache_dir {
            None => BgpkitParser::new(url).map_err(|e| PyValueError::new_err(e.to_string())),
            Some(dir) => {
                BgpkitParser::new_cached(url, dir).map_err(|e| PyValueError::new_err(e.to_string()))
            }
        }
    }

    fn option_to_string<T: ToString>(v: &Option<T>) -> String {
        v.as_ref().map(|x| x.to_string()).unwrap_or_default()
    }

    fn option_vec_to_string<T: ToString>(v: &Option<Vec<T>>) -> String {
        v.as_ref()
            .map(|items| {
                items
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default()
    }

    #[derive(Clone, Copy)]
    enum ElemField {
        Timestamp,
        ElemType,
        PeerIp,
        PeerAsn,
        PeerBgpId,
        Prefix,
        NextHop,
        AsPath,
        OriginAsns,
        OriginAsn,
        Origin,
        LocalPref,
        Med,
        Communities,
        Atomic,
        AggrAsn,
        AggrIp,
        OnlyToCustomer,
    }

    #[derive(Clone, Copy)]
    enum RouteField {
        Timestamp,
        ElemType,
        PeerIp,
        PeerAsn,
        Prefix,
        AsPath,
    }

    fn parse_elem_field(field: &str) -> PyResult<ElemField> {
        match field {
            "timestamp" => Ok(ElemField::Timestamp),
            "elem_type" | "type" => Ok(ElemField::ElemType),
            "peer_ip" => Ok(ElemField::PeerIp),
            "peer_asn" => Ok(ElemField::PeerAsn),
            "peer_bgp_id" => Ok(ElemField::PeerBgpId),
            "prefix" => Ok(ElemField::Prefix),
            "next_hop" => Ok(ElemField::NextHop),
            "as_path" => Ok(ElemField::AsPath),
            "origin_asns" => Ok(ElemField::OriginAsns),
            "origin_asn" => Ok(ElemField::OriginAsn),
            "origin" => Ok(ElemField::Origin),
            "local_pref" => Ok(ElemField::LocalPref),
            "med" => Ok(ElemField::Med),
            "communities" => Ok(ElemField::Communities),
            "atomic" => Ok(ElemField::Atomic),
            "aggr_asn" => Ok(ElemField::AggrAsn),
            "aggr_ip" => Ok(ElemField::AggrIp),
            "only_to_customer" => Ok(ElemField::OnlyToCustomer),
            _ => Err(PyValueError::new_err(format!("unknown field: {field}"))),
        }
    }

    fn parse_route_field(field: &str) -> PyResult<RouteField> {
        match field {
            "timestamp" => Ok(RouteField::Timestamp),
            "elem_type" | "type" => Ok(RouteField::ElemType),
            "peer_ip" => Ok(RouteField::PeerIp),
            "peer_asn" => Ok(RouteField::PeerAsn),
            "prefix" => Ok(RouteField::Prefix),
            "as_path" => Ok(RouteField::AsPath),
            _ => Err(PyValueError::new_err(format!(
                "unknown route field: {field}"
            ))),
        }
    }

    fn parse_elem_fields(fields: Vec<String>) -> PyResult<Vec<ElemField>> {
        if fields.is_empty() {
            return Err(PyValueError::new_err("fields must not be empty"));
        }
        fields
            .iter()
            .map(|f| parse_elem_field(f.as_str()))
            .collect()
    }

    fn parse_route_fields(fields: Vec<String>) -> PyResult<Vec<RouteField>> {
        if fields.is_empty() {
            return Err(PyValueError::new_err("fields must not be empty"));
        }
        fields
            .iter()
            .map(|f| parse_route_field(f.as_str()))
            .collect()
    }

    fn elem_field_to_py(py: Python, elem: &BgpElem, field: ElemField) -> PyResult<Py<PyAny>> {
        match field {
            ElemField::Timestamp => elem.timestamp.into_py_any(py),
            ElemField::ElemType => match elem.elem_type {
                ElemType::ANNOUNCE => "A".into_py_any(py),
                ElemType::WITHDRAW => "W".into_py_any(py),
            },
            ElemField::PeerIp => elem.peer_ip.to_string().into_py_any(py),
            ElemField::PeerAsn => elem.peer_asn.to_u32().into_py_any(py),
            ElemField::PeerBgpId => elem.peer_bgp_id.map(|v| v.to_string()).into_py_any(py),
            ElemField::Prefix => elem.prefix.to_string().into_py_any(py),
            ElemField::NextHop => elem.next_hop.map(|v| v.to_string()).into_py_any(py),
            ElemField::AsPath => elem.as_path.as_ref().map(|v| v.to_string()).into_py_any(py),
            ElemField::OriginAsns => elem
                .origin_asns
                .as_ref()
                .map(|v| v.iter().map(|x| x.to_u32()).collect::<Vec<_>>())
                .into_py_any(py),
            ElemField::OriginAsn => elem
                .origin_asns
                .as_ref()
                .and_then(|origin_asns| (origin_asns.len() == 1).then_some(origin_asns[0].to_u32()))
                .into_py_any(py),
            ElemField::Origin => elem.origin.map(|v| v.to_string()).into_py_any(py),
            ElemField::LocalPref => elem.local_pref.into_py_any(py),
            ElemField::Med => elem.med.into_py_any(py),
            ElemField::Communities => elem
                .communities
                .as_ref()
                .map(|v| v.iter().map(|x| x.to_string()).collect::<Vec<_>>())
                .into_py_any(py),
            ElemField::Atomic => elem.atomic.into_py_any(py),
            ElemField::AggrAsn => elem.aggr_asn.map(|v| v.to_u32()).into_py_any(py),
            ElemField::AggrIp => elem.aggr_ip.map(|v| v.to_string()).into_py_any(py),
            ElemField::OnlyToCustomer => elem.only_to_customer.map(|v| v.to_u32()).into_py_any(py),
        }
    }

    fn route_field_to_py(
        py: Python,
        elem: &BgpRouteElem,
        field: RouteField,
    ) -> PyResult<Py<PyAny>> {
        match field {
            RouteField::Timestamp => elem.timestamp.into_py_any(py),
            RouteField::ElemType => match elem.elem_type {
                ElemType::ANNOUNCE => "A".into_py_any(py),
                ElemType::WITHDRAW => "W".into_py_any(py),
            },
            RouteField::PeerIp => elem.peer_ip.to_string().into_py_any(py),
            RouteField::PeerAsn => elem.peer_asn.to_u32().into_py_any(py),
            RouteField::Prefix => elem.prefix.to_string().into_py_any(py),
            RouteField::AsPath => elem.as_path.as_ref().map(|v| v.to_string()).into_py_any(py),
        }
    }

    fn elem_to_tuple(py: Python, elem: BgpElem, fields: &[ElemField]) -> PyResult<Py<PyTuple>> {
        let values = fields
            .iter()
            .map(|field| elem_field_to_py(py, &elem, *field))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyTuple::new(py, values)?.unbind())
    }

    fn route_to_tuple(
        py: Python,
        elem: BgpRouteElem,
        fields: &[RouteField],
    ) -> PyResult<Py<PyTuple>> {
        let values = fields
            .iter()
            .map(|field| route_field_to_py(py, &elem, *field))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyTuple::new(py, values)?.unbind())
    }

    #[pyclass(skip_from_py_object)]
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
        pub peer_bgp_id: Option<String>,
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
        #[pyo3(get, set)]
        pub only_to_customer: Option<u32>,
    }

    #[pymethods]
    impl Elem {
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
            use pyo3::types::PyDict;
            let dict = PyDict::new(py);
            dict.set_item("timestamp", self.timestamp)?;
            dict.set_item("elem_type", self.elem_type.clone())?;
            dict.set_item("peer_ip", self.peer_ip.clone())?;
            dict.set_item("peer_asn", self.peer_asn)?;
            dict.set_item("peer_bgp_id", self.peer_bgp_id.clone())?;
            dict.set_item("prefix", self.prefix.clone())?;
            dict.set_item("next_hop", self.next_hop.clone())?;
            dict.set_item("as_path", self.as_path.clone())?;
            dict.set_item("origin_asns", self.origin_asns.clone())?;
            dict.set_item("origin", self.origin.clone())?;
            dict.set_item("local_pref", self.local_pref)?;
            dict.set_item("med", self.med)?;
            dict.set_item("communities", self.communities.clone())?;
            dict.set_item("atomic", self.atomic.clone())?;
            dict.set_item("aggr_asn", self.aggr_asn)?;
            dict.set_item("aggr_ip", self.aggr_ip.clone())?;
            dict.set_item("only_to_customer", self.only_to_customer)?;
            Ok(dict)
        }

        pub fn as_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
            self.to_dict(py)
        }

        #[getter(origin_asn)]
        fn origin_asn_value(&self) -> Option<u32> {
            self.get_origin_asn()
        }

        fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
            self.to_dict(py)
        }

        #[pyo3(name = "__str__")]
        fn str_repr(&self) -> PyResult<String> {
            self.to_json()
        }

        #[pyo3(name = "__repr__")]
        fn repr(&self) -> PyResult<String> {
            Ok(format!(
                "<Elem prefix={} peer={} type={}>",
                self.prefix, self.peer_ip, self.elem_type
            ))
        }

        pub fn is_announcement(&self) -> bool {
            self.elem_type.eq_ignore_ascii_case("A")
        }

        pub fn is_withdrawal(&self) -> bool {
            self.elem_type.eq_ignore_ascii_case("W")
        }

        pub fn get_origin_asn(&self) -> Option<u32> {
            self.origin_asns
                .as_ref()
                .and_then(|origin_asns| (origin_asns.len() == 1).then_some(origin_asns[0]))
        }

        pub fn get_origin_asns(&self) -> Option<Vec<u32>> {
            self.origin_asns.clone()
        }

        pub fn has_as_path(&self) -> bool {
            self.as_path.is_some()
        }

        pub fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
        }

        #[staticmethod]
        pub fn get_psv_header() -> String {
            [
                "type",
                "timestamp",
                "peer_ip",
                "peer_asn",
                "prefix",
                "as_path",
                "origin_asns",
                "origin",
                "next_hop",
                "local_pref",
                "med",
                "communities",
                "atomic",
                "aggr_asn",
                "aggr_ip",
                "only_to_customer",
            ]
            .join("|")
        }

        pub fn to_psv(&self) -> String {
            format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                self.elem_type,
                self.timestamp,
                self.peer_ip,
                self.peer_asn,
                self.prefix,
                option_to_string(&self.as_path),
                option_vec_to_string(&self.origin_asns),
                option_to_string(&self.origin),
                option_to_string(&self.next_hop),
                option_to_string(&self.local_pref),
                option_to_string(&self.med),
                option_vec_to_string(&self.communities),
                option_to_string(&self.atomic),
                option_to_string(&self.aggr_asn),
                option_to_string(&self.aggr_ip),
                option_to_string(&self.only_to_customer),
            )
        }
    }

    #[pyclass(skip_from_py_object)]
    #[derive(Clone, PartialEq, Serialize)]
    pub struct RouteElem {
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
        pub as_path: Option<String>,
    }

    #[pymethods]
    impl RouteElem {
        pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
            use pyo3::types::PyDict;
            let dict = PyDict::new(py);
            dict.set_item("timestamp", self.timestamp)?;
            dict.set_item("elem_type", self.elem_type.clone())?;
            dict.set_item("peer_ip", self.peer_ip.clone())?;
            dict.set_item("peer_asn", self.peer_asn)?;
            dict.set_item("prefix", self.prefix.clone())?;
            dict.set_item("as_path", self.as_path.clone())?;
            Ok(dict)
        }

        pub fn as_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
            self.to_dict(py)
        }

        pub fn is_announcement(&self) -> bool {
            self.elem_type.eq_ignore_ascii_case("A")
        }

        pub fn is_withdrawal(&self) -> bool {
            self.elem_type.eq_ignore_ascii_case("W")
        }

        pub fn has_as_path(&self) -> bool {
            self.as_path.is_some()
        }

        pub fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
        }

        #[pyo3(name = "__str__")]
        fn str_repr(&self) -> PyResult<String> {
            self.to_json()
        }

        #[pyo3(name = "__repr__")]
        fn repr(&self) -> PyResult<String> {
            Ok(format!(
                "<RouteElem prefix={} peer={} type={}>",
                self.prefix, self.peer_ip, self.elem_type
            ))
        }
    }

    #[pyclass(name = "Filter", skip_from_py_object)]
    #[derive(Clone)]
    struct PyFilter {
        inner: BgpkitFilter,
    }

    #[pymethods]
    impl PyFilter {
        #[new]
        #[pyo3(signature = (filter_type, filter_value))]
        fn new(filter_type: String, filter_value: String) -> PyResult<Self> {
            Self::from_parts(filter_type.as_str(), filter_value.as_str())
        }

        #[staticmethod]
        fn peer_ip(peer_ip: String) -> PyResult<Self> {
            Self::from_parts("peer_ip", peer_ip.as_str())
        }

        #[staticmethod]
        fn peer_ips(peer_ips: Vec<String>) -> PyResult<Self> {
            Self::from_parts("peer_ips", peer_ips.join(",").as_str())
        }

        #[staticmethod]
        fn origin_asn(origin_asn: u32) -> PyResult<Self> {
            Self::from_parts("origin_asn", origin_asn.to_string().as_str())
        }

        #[staticmethod]
        fn prefix(prefix: String) -> PyResult<Self> {
            Self::from_parts("prefix", prefix.as_str())
        }

        #[staticmethod]
        fn elem_type(elem_type: String) -> PyResult<Self> {
            Self::from_parts("type", elem_type.as_str())
        }

        #[pyo3(name = "__repr__")]
        fn repr(&self) -> PyResult<String> {
            Ok(format!("<Filter {:?}>", self.inner))
        }
    }

    impl PyFilter {
        fn from_parts(filter_type: &str, filter_value: &str) -> PyResult<Self> {
            let inner = BgpkitFilter::new(filter_type, filter_value)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(PyFilter { inner })
        }
    }

    #[pyclass(unsendable)]
    struct Parser {
        elem_iter: Option<ElemIterator<Box<dyn Send + Read>>>,
    }

    #[pyclass(unsendable)]
    struct BatchIterator {
        elem_iter: Option<ElemIterator<Box<dyn Send + Read>>>,
        batch_size: usize,
    }

    #[pyclass(unsendable)]
    struct RouteParser {
        route_iter: Option<RouteIterator<Box<dyn Send + Read>>>,
    }

    #[pyclass(unsendable)]
    struct RouteBatchIterator {
        route_iter: Option<RouteIterator<Box<dyn Send + Read>>>,
        batch_size: usize,
    }

    #[pyclass(unsendable)]
    struct TupleIterator {
        elem_iter: Option<ElemIterator<Box<dyn Send + Read>>>,
        fields: Vec<ElemField>,
    }

    #[pyclass(unsendable)]
    struct TupleBatchIterator {
        elem_iter: Option<ElemIterator<Box<dyn Send + Read>>>,
        fields: Vec<ElemField>,
        batch_size: usize,
    }

    #[pyclass(unsendable)]
    struct RouteTupleIterator {
        route_iter: Option<RouteIterator<Box<dyn Send + Read>>>,
        fields: Vec<RouteField>,
    }

    #[pyclass(unsendable)]
    struct RouteTupleBatchIterator {
        route_iter: Option<RouteIterator<Box<dyn Send + Read>>>,
        fields: Vec<RouteField>,
        batch_size: usize,
    }

    #[pymethods]
    impl Parser {
        #[new]
        #[pyo3(signature = (url, filters=None, cache_dir=None))]
        fn new(
            url: String,
            filters: Option<HashMap<String, String>>,
            cache_dir: Option<String>,
        ) -> PyResult<Self> {
            let mut parser = new_parser(url.as_str(), cache_dir.as_deref())?;

            if let Some(filters) = filters {
                for (k, v) in filters {
                    parser = parser
                        .add_filter(k.as_str(), v.as_str())
                        .map_err(|e| PyValueError::new_err(e.to_string()))?;
                }
            }
            let elem_iter = Some(parser.into_elem_iter());
            Ok(Parser { elem_iter })
        }

        #[staticmethod]
        #[pyo3(signature = (url, filters, cache_dir=None))]
        fn from_filters(
            url: String,
            filters: Vec<PyRef<PyFilter>>,
            cache_dir: Option<String>,
        ) -> PyResult<Self> {
            let parser = new_parser(url.as_str(), cache_dir.as_deref())?;
            let filters = filters
                .iter()
                .map(|f| f.inner.clone())
                .collect::<Vec<BgpkitFilter>>();
            let elem_iter = Some(parser.with_filters(&filters).into_elem_iter());
            Ok(Parser { elem_iter })
        }

        fn parse_all(&mut self, py: Python) -> PyResult<Vec<Py<Elem>>> {
            let Some(mut elem_iter) = self.elem_iter.take() else {
                return Ok(Vec::new());
            };
            let elems = py.detach(|| elem_iter.by_ref().map(convert_elem).collect::<Vec<Elem>>());
            elems.into_iter().map(|e| Py::new(py, e)).collect()
        }

        fn parse_next(&mut self, py: Python) -> PyResult<Option<Py<Elem>>> {
            let Some(elem_iter) = self.elem_iter.as_mut() else {
                return Ok(None);
            };
            elem_iter
                .next()
                .map(|e| Py::new(py, convert_elem(e)))
                .transpose()
        }

        fn count(&mut self, py: Python) -> usize {
            let Some(elem_iter) = self.elem_iter.take() else {
                return 0;
            };
            py.detach(|| elem_iter.count())
        }

        fn iter_batches(&mut self, batch_size: usize) -> PyResult<BatchIterator> {
            if batch_size == 0 {
                return Err(PyValueError::new_err("batch_size must be greater than 0"));
            }
            Ok(BatchIterator {
                elem_iter: self.elem_iter.take(),
                batch_size,
            })
        }

        fn iter_tuples(&mut self, fields: Vec<String>) -> PyResult<TupleIterator> {
            Ok(TupleIterator {
                elem_iter: self.elem_iter.take(),
                fields: parse_elem_fields(fields)?,
            })
        }

        fn iter_tuple_batches(
            &mut self,
            fields: Vec<String>,
            batch_size: usize,
        ) -> PyResult<TupleBatchIterator> {
            if batch_size == 0 {
                return Err(PyValueError::new_err("batch_size must be greater than 0"));
            }
            Ok(TupleBatchIterator {
                elem_iter: self.elem_iter.take(),
                fields: parse_elem_fields(fields)?,
                batch_size,
            })
        }

        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<Elem>>> {
            let Some(elem_iter) = slf.elem_iter.as_mut() else {
                return Ok(None);
            };
            elem_iter
                .next()
                .map(|e| Py::new(py, convert_elem(e)))
                .transpose()
        }
    }

    #[pymethods]
    impl BatchIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Vec<Py<Elem>>>> {
            let batch_size = slf.batch_size;
            let Some(elem_iter) = slf.elem_iter.as_mut() else {
                return Ok(None);
            };

            let elems = py.detach(|| {
                elem_iter
                    .by_ref()
                    .take(batch_size)
                    .map(convert_elem)
                    .collect::<Vec<Elem>>()
            });

            if elems.is_empty() {
                slf.elem_iter = None;
                return Ok(None);
            }

            elems
                .into_iter()
                .map(|e| Py::new(py, e))
                .collect::<PyResult<Vec<_>>>()
                .map(Some)
        }
    }

    #[pymethods]
    impl TupleIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<PyTuple>>> {
            let slf = &mut *slf;
            let fields = slf.fields.as_slice();
            let Some(elem_iter) = slf.elem_iter.as_mut() else {
                return Ok(None);
            };
            elem_iter
                .next()
                .map(|elem| elem_to_tuple(py, elem, fields))
                .transpose()
        }
    }

    #[pymethods]
    impl TupleBatchIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Vec<Py<PyTuple>>>> {
            let slf = &mut *slf;
            let fields = slf.fields.as_slice();
            let batch_size = slf.batch_size;
            let Some(elem_iter) = slf.elem_iter.as_mut() else {
                return Ok(None);
            };

            let elems = py.detach(|| {
                elem_iter
                    .by_ref()
                    .take(batch_size)
                    .collect::<Vec<BgpElem>>()
            });
            if elems.is_empty() {
                slf.elem_iter = None;
                return Ok(None);
            }

            elems
                .into_iter()
                .map(|elem| elem_to_tuple(py, elem, fields))
                .collect::<PyResult<Vec<_>>>()
                .map(Some)
        }
    }

    #[pymethods]
    impl RouteParser {
        #[new]
        #[pyo3(signature = (url, filters=None, cache_dir=None))]
        fn new(
            url: String,
            filters: Option<HashMap<String, String>>,
            cache_dir: Option<String>,
        ) -> PyResult<Self> {
            let mut parser = new_parser(url.as_str(), cache_dir.as_deref())?;

            if let Some(filters) = filters {
                for (k, v) in filters {
                    parser = parser
                        .add_filter(k.as_str(), v.as_str())
                        .map_err(|e| PyValueError::new_err(e.to_string()))?;
                }
            }
            let route_iter = Some(parser.into_route_iter());
            Ok(RouteParser { route_iter })
        }

        #[staticmethod]
        #[pyo3(signature = (url, filters, cache_dir=None))]
        fn from_filters(
            url: String,
            filters: Vec<PyRef<PyFilter>>,
            cache_dir: Option<String>,
        ) -> PyResult<Self> {
            let parser = new_parser(url.as_str(), cache_dir.as_deref())?;
            let filters = filters
                .iter()
                .map(|f| f.inner.clone())
                .collect::<Vec<BgpkitFilter>>();
            let route_iter = Some(parser.with_filters(&filters).into_route_iter());
            Ok(RouteParser { route_iter })
        }

        fn parse_all(&mut self, py: Python) -> PyResult<Vec<Py<RouteElem>>> {
            let Some(mut route_iter) = self.route_iter.take() else {
                return Ok(Vec::new());
            };
            let routes = py.detach(|| {
                route_iter
                    .by_ref()
                    .map(convert_route_elem)
                    .collect::<Vec<RouteElem>>()
            });
            routes.into_iter().map(|e| Py::new(py, e)).collect()
        }

        fn parse_next(&mut self, py: Python) -> PyResult<Option<Py<RouteElem>>> {
            let Some(route_iter) = self.route_iter.as_mut() else {
                return Ok(None);
            };
            route_iter
                .next()
                .map(|e| Py::new(py, convert_route_elem(e)))
                .transpose()
        }

        fn count(&mut self, py: Python) -> usize {
            let Some(route_iter) = self.route_iter.take() else {
                return 0;
            };
            py.detach(|| route_iter.count())
        }

        fn iter_batches(&mut self, batch_size: usize) -> PyResult<RouteBatchIterator> {
            if batch_size == 0 {
                return Err(PyValueError::new_err("batch_size must be greater than 0"));
            }
            Ok(RouteBatchIterator {
                route_iter: self.route_iter.take(),
                batch_size,
            })
        }

        fn iter_tuples(&mut self, fields: Vec<String>) -> PyResult<RouteTupleIterator> {
            Ok(RouteTupleIterator {
                route_iter: self.route_iter.take(),
                fields: parse_route_fields(fields)?,
            })
        }

        fn iter_tuple_batches(
            &mut self,
            fields: Vec<String>,
            batch_size: usize,
        ) -> PyResult<RouteTupleBatchIterator> {
            if batch_size == 0 {
                return Err(PyValueError::new_err("batch_size must be greater than 0"));
            }
            Ok(RouteTupleBatchIterator {
                route_iter: self.route_iter.take(),
                fields: parse_route_fields(fields)?,
                batch_size,
            })
        }

        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<RouteElem>>> {
            let Some(route_iter) = slf.route_iter.as_mut() else {
                return Ok(None);
            };
            route_iter
                .next()
                .map(|e| Py::new(py, convert_route_elem(e)))
                .transpose()
        }
    }

    #[pymethods]
    impl RouteTupleIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<PyTuple>>> {
            let slf = &mut *slf;
            let fields = slf.fields.as_slice();
            let Some(route_iter) = slf.route_iter.as_mut() else {
                return Ok(None);
            };
            route_iter
                .next()
                .map(|route| route_to_tuple(py, route, fields))
                .transpose()
        }
    }

    #[pymethods]
    impl RouteTupleBatchIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Vec<Py<PyTuple>>>> {
            let slf = &mut *slf;
            let fields = slf.fields.as_slice();
            let batch_size = slf.batch_size;
            let Some(route_iter) = slf.route_iter.as_mut() else {
                return Ok(None);
            };

            let routes = py.detach(|| {
                route_iter
                    .by_ref()
                    .take(batch_size)
                    .collect::<Vec<BgpRouteElem>>()
            });
            if routes.is_empty() {
                slf.route_iter = None;
                return Ok(None);
            }

            routes
                .into_iter()
                .map(|route| route_to_tuple(py, route, fields))
                .collect::<PyResult<Vec<_>>>()
                .map(Some)
        }
    }

    #[pymethods]
    impl RouteBatchIterator {
        fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Vec<Py<RouteElem>>>> {
            let batch_size = slf.batch_size;
            let Some(route_iter) = slf.route_iter.as_mut() else {
                return Ok(None);
            };

            let routes = py.detach(|| {
                route_iter
                    .by_ref()
                    .take(batch_size)
                    .map(convert_route_elem)
                    .collect::<Vec<RouteElem>>()
            });

            if routes.is_empty() {
                slf.route_iter = None;
                return Ok(None);
            }

            routes
                .into_iter()
                .map(|e| Py::new(py, e))
                .collect::<PyResult<Vec<_>>>()
                .map(Some)
        }
    }

    m.add_class::<Elem>()?;
    m.add_class::<RouteElem>()?;
    m.add_class::<PyFilter>()?;
    m.add_class::<Parser>()?;
    m.add_class::<BatchIterator>()?;
    m.add_class::<RouteParser>()?;
    m.add_class::<RouteBatchIterator>()?;
    m.add_class::<TupleIterator>()?;
    m.add_class::<TupleBatchIterator>()?;
    m.add_class::<RouteTupleIterator>()?;
    m.add_class::<RouteTupleBatchIterator>()?;
    m.add("ELEM_TYPE_ANNOUNCE", "A")?;
    m.add("ELEM_TYPE_WITHDRAW", "W")?;
    m.add("PSV_HEADER", Elem::get_psv_header())?;
    m.add(
        "BASIC_FIELDS",
        vec!["timestamp", "elem_type", "peer_ip", "peer_asn", "prefix"],
    )?;
    m.add(
        "ROUTE_FIELDS",
        vec![
            "timestamp",
            "elem_type",
            "peer_ip",
            "peer_asn",
            "prefix",
            "as_path",
        ],
    )?;
    m.add(
        "NEXT_HOP_FIELDS",
        vec![
            "timestamp",
            "elem_type",
            "peer_ip",
            "peer_asn",
            "prefix",
            "next_hop",
        ],
    )?;
    Ok(())
}
