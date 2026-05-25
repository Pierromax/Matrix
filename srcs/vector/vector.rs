use std::fmt;

pub struct Vector<K> 
{
    pub(crate) data: Vec<K>,
}

/**
 * implementation du trait Form pour creation de Vector
 */
impl<K: Clone, const C: usize> From<[K; C]> for Vector<K> 
{
    fn from(arr: [K; C]) -> Self 
    {
        let mut data: Vec<K> = Vec::with_capacity(C);
        for elem in &arr{
            data.push(elem.clone());
        }
         Vector{ 
            data,
        }
    }
}

/**
 * Implementation de la methode d'affiche de la matrice
*/
impl<K: fmt::Display> fmt::Display for Vector<K>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for elem in &self.data {
            write!(f, "[{}]\n", elem)?
        }
        Ok(())
    }
}

impl<K: Clone> Clone for Vector<K> 
{
    fn clone(&self) -> Self {
        Vector { data: self.data.clone() }
    }
}

pub trait Len {
    fn len(&self) -> usize;
}

impl<K> Len for Vector<K>
{
    fn len(&self) -> usize{
        self.data.len()
    }
}