use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use pyo3::PyIterProtocol;
use pyo3::exceptions;
use pyo3::types::{PyTuple, PyFloat, PyBool};
use pyo3::class::number::PyNumberProtocol;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::class::basic::CompareOp;


#[pyclass]
#[derive(Copy, Clone)]
struct Vector2 {
    #[pyo3(get)]
    x: f64,

    #[pyo3(get)]
    y: f64,
}


/// Get the zero vector
fn zero() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
}


#[pymethods]
impl Vector2 {
    #[new]
    fn new(x: f64, y: f64) -> PyResult<Self> {
        if x.is_finite() && y.is_finite() {
            Ok(Vector2 { x, y })
        } else {
            Err(exceptions::ValueError::py_err(
                "x/y values may not be NaN/inf"
            ))
        }
    }

    #[staticmethod]
    fn from_polar(r: f64, theta: f64) -> PyResult<Self> {
        let x = r * theta.cos();
        let y = r * theta.sin();
        Ok(Vector2 {x, y})
    }

    fn is_zero(&self) -> bool {
        return self.x == 0.0 && self.y == 0.0
    }

    fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    fn to_polar(&self) -> (f64, f64) {
        (self.length(), self.angle())
    }

    fn normalized(&self) -> Self {
        if self.is_zero() {
            return Vector2 { x: 1.0, y: 0.0 }
        }
        let mag = self.length();

        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}


#[pyproto]
impl PyObjectProtocol for Vector2 {
    fn __repr__(&self) -> String {
        format!("v({}, {})", self.x, self.y)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyObject {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();


        let cmp: bool = match op {
            CompareOp::Eq => false,
            CompareOp::Ne => true,
            _ => {
                return py.NotImplemented();
            }
        };

        if let Ok(v) = other.extract::<Vector2>() {
            let eq = v.x == self.x && v.y == self.y;
            return PyBool::new(py, eq ^ cmp).into();
        }

        match other.extract::<Vec<f64>>() {
            Ok(vals) => {
                let eq = vals.len() == 2
                         && vals[0] == self.x
                         && vals[1] == self.y;
                PyBool::new(py, eq ^ cmp).into()
            },
            Err(_) => {
                py.NotImplemented()
            }
        }
    }
}


#[pyproto]
impl PyNumberProtocol for Vector2 {
    fn __add__(lhs: PyRef<'p, Vector2>, rhs: PyRef<'p, Vector2>) -> Vector2 {
        Vector2 {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
        }
    }
}


#[pyproto]
impl PySequenceProtocol for Vector2 {
    fn __len__(&self) -> usize {
        2
    }
}


#[pyclass]
struct VecIter {
    v: Vector2,
    pos: usize,
}


#[pyproto]
impl PyIterProtocol for VecIter {
    fn __iter__(slf: PyRef<Self>) -> Py<VecIter> {
        slf.into()
    }
    fn __next__(mut slf: PyRefMut<Self>) -> Option<f64> {
        let res = match slf.pos {
            0 => Some(slf.v.x),
            1 => Some(slf.v.y),
            _ => None,
        };
        slf.pos += 1;
        res
    }
}


#[pyproto]
impl PyIterProtocol for Vector2 {
    fn __iter__(slf: PyRef<Self>) -> VecIter {
        VecIter {
            v: slf.clone(),
            pos: 0
        }
    }
}



#[pymodule]
fn wvec(_py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our Python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values, and the Rust return value back into a Python object.
    // The `_py` argument represents that we're holding the GIL.
    #[pyfn(m, "sum_as_string")]
    fn sum_as_string_py(_py: Python, a: i64, b: i64) -> PyResult<String> {
        let out = sum_as_string(a, b);
        Ok(out)
    }

    m.add_class::<Vector2>()?;

    Ok(())
}

// logic implemented as a normal Rust function
fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b)
}
