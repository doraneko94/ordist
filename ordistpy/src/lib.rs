use pyo3::prelude::*;
use pyo3::types::{PyList};

use ordist::{corrcoef, distances, lcs};

macro_rules! ordist_py {
    ($name:ident, $type:ty) => {
        #[pyclass]
        struct $name {
            v1: Vec<$type>,
            v2: Vec<$type>,
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new(v1: &PyList, v2: &PyList) -> Self {
                let v1 = v1.extract().unwrap();
                let v2 = v2.extract().unwrap();
                Self { v1, v2 }
            }
            fn spearman_dist(&self) -> usize {
                distances::spearman_dist(&self.v1, &self.v2).unwrap()
            }
            fn footrule_dist(&self) -> usize {
                distances::footrule_dist(&self.v1, &self.v2).unwrap()
            }
            fn kendall_dist(&self) -> usize {
                distances::kendall_dist(&self.v1, &self.v2).unwrap()
            }
            fn cayley_dist(&self) -> usize {
                distances::cayley_dist(&self.v1, &self.v2).unwrap()
            }
            fn levenshtein_dist(&self) -> usize {
                distances::levenshtein_dist(&self.v1, &self.v2).unwrap()
            }
            fn ulam_dist(&self) -> usize {
                distances::ulam_dist(&self.v1, &self.v2).unwrap()
            }
            fn spearman_corrcoef(&self) -> f64 {
                corrcoef::spearman_corrcoef(&self.v1, &self.v2).unwrap()
            }
            fn kendall_corrcoef(&self) -> f64 {
                corrcoef::kendall_corrcoef(&self.v1, &self.v2).unwrap()
            }
            fn lcs(&self) -> usize {
                lcs::lcs(&self.v1, &self.v2)
            }
            fn lcs_with_list(&self) -> (usize, Vec<$type>) {
                lcs::lcs_with_vec(&self.v1, &self.v2)
            }
        }
    };
} // ordist_py

ordist_py!(OrDistInt, i32);
ordist_py!(OrDistStr, String);

#[pymodule]
fn ordist(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<OrDistInt>()?;
    m.add_class::<OrDistStr>()?;
    Ok(())
}