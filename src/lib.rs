use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use pyo3::PyIterProtocol;
use pyo3::exceptions;
use pyo3::types::PyBool;
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

    /// Construct a new cartesian vector from r (length) and theta (angle).
    #[text_signature = "(r: float, theta: float)"]
    #[staticmethod]
    fn from_polar(r: f64, theta: f64) -> PyResult<Self> {
        let x = r * theta.cos();
        let y = r * theta.sin();
        Ok(Vector2 {x, y})
    }

    /// Return True if this vector is the zero vector.
    ///
    /// Note that bool(vec) will always return True, because a Vector2 is a
    /// sequence of length 2.
    fn is_zero(&self) -> bool {
        return self.x == 0.0 && self.y == 0.0
    }

    /// Return the length of the vector, squared.
    ///
    /// This is minutely faster than getting the length and is sufficient for
    /// some comparison purposes.
    fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    /// Return the length of the vector.
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Return the angle this vector makes to the positive x axis.
    fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Return a tuple (r, theta) giving a polar representation of this vector.
    fn to_polar(&self) -> (f64, f64) {
        (self.length(), self.angle())
    }

    /// Return a normalized copy of this vector.
    ///
    /// If the vector is of zero length then an arbitrary zero-length vector
    /// is returned.
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
        format!("Vector2({}, {})", self.x, self.y)
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

   fn __mul__(lhs: PyRef<'p, Vector2>, rhs: f64) -> Vector2 {
        Vector2 {
            x: lhs.x * rhs,
            y: lhs.y * rhs,
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
    m.add_class::<Vector2>()?;

    Ok(())
}
