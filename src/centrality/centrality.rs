use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::Index;

/// A trait for the centrality score of a single paper
///
/// Different centrality scores may be of different types. For example,
/// degree centrality scores are integers, whereas page ranks are floats.
pub(crate) trait Centrality<T>: PartialOrd + Display + Clone {
    /// The id of the paper
    fn vertex(&self) -> usize;
    /// The centrality score
    fn score(&self) -> T;
}

/// A data structure that stores the centrality scores of a network
pub(crate) struct CentralityRank<T, U: Centrality<T>> {
    ranks: Vec<U>,
    phantom: PhantomData<T>,
}

impl<T, U: Centrality<T>> CentralityRank<T, U> {
    pub(crate) fn new(ranks: Vec<U>) -> CentralityRank<T, U> {
        CentralityRank {
            ranks,
            phantom: PhantomData,
        }
    }
}

impl<T, U: Centrality<T>> Index<usize> for CentralityRank<T, U> {
    type Output = U;

    fn index(&self, index: usize) -> &Self::Output {
        self.ranks.get(index).unwrap()
    }
}

impl<T, U: Centrality<T>> CentralityRank<T, U> {
    /// Returns the vertices with the top centrality scores in a network
    ///
    /// # Arguments
    ///
    /// * `n` - The number of centrality scores to return
    pub(crate) fn top(&self, n: usize) -> CentralityRank<T, U> {
        CentralityRank {
            ranks: self.ranks.iter().cloned().take(n).collect(),
            phantom: PhantomData,
        }
    }
}

impl<T, U: Centrality<T>> Display for CentralityRank<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in &self.ranks {
            writeln!(f, "{}", rank)?;
        }
        Ok(())
    }
}
