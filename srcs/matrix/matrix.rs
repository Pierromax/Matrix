use std::fmt;

pub struct Matrix<K> 
{
    pub(crate) data: Vec<K>,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
}

/**
 * implementation du trait Form pour creation de Matrix
 */
impl<K: Clone, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> 
{
    fn from(arr: [[K; C]; R]) -> Self 
    {
        let mut data: Vec<K> = Vec::with_capacity(R*C);

        for row in &arr{
            for elem in row{
                data.push(elem.clone());
            }
        }

        Matrix {
            data,
            rows: R,
            cols: C,
        }
    }
}

/**
 * Implementation de la methode d'affiche de la matrice
*/
impl<K: fmt::Display> fmt::Display for Matrix<K>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for row in 0..self.rows {
            write!(f,"[")?;
            for col in 0..self.cols {
                write!(f, "{}{}", 
                    self.data[row * self.cols + col],
                    if col < self.cols - 1 {", "} else {"]\n"}
                )?;
            }
        }
        Ok(())
    }
}

impl<K: Clone> Clone for Matrix<K> 
{
    fn clone(&self) -> Self {
        Matrix {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}