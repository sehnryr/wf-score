use std::iter::Iterator;
use std::slice::Iter;
use std::sync::Arc;

use itertools::{Combinations, Itertools, Permutations, Product};
use wf_stats::Modifier;

pub struct ModCombinations<'a, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    mods_iterator_with_riven:
        Option<Product<Product<Permutations<Iter<'a, T>>, Combinations<Iter<'a, T>>>, Iter<'a, T>>>,
    mods_iterator_without_riven:
        Option<Product<Permutations<Iter<'a, T>>, Combinations<Iter<'a, T>>>>,
    obligatory_mods: &'a [T],
}

impl<'a, T> ModCombinations<'a, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    pub fn new(
        status_mod_count: usize,
        mod_count: usize,
        status_mods: &'a [T],
        other_mods: &'a [T],
        riven_mods: &'a [T],
        obligatory_mods: &'a [T],
    ) -> Self {
        let are_riven_mods = riven_mods.len() > 0;

        let status_mods_iterator = status_mods.iter().permutations(status_mod_count);
        let other_mods_iterator = other_mods.iter().combinations(
            (mod_count as isize
                - status_mod_count as isize
                - obligatory_mods.len() as isize
                - if are_riven_mods { 1 } else { 0 })
            .max(0) as usize,
        );
        let riven_mods_iterator = riven_mods.iter();

        let (mods_iterator_with_riven, mods_iterator_without_riven) = if are_riven_mods {
            let iter = status_mods_iterator
                .cartesian_product(other_mods_iterator)
                .cartesian_product(riven_mods_iterator);
            (Some(iter), None)
        } else {
            let iter = status_mods_iterator.cartesian_product(other_mods_iterator);
            (None, Some(iter))
        };

        Self {
            mods_iterator_with_riven,
            mods_iterator_without_riven,
            obligatory_mods,
        }
    }
}

impl<'a, T> Iterator for ModCombinations<'a, T>
where
    T: Clone + Into<Arc<dyn Modifier>>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.mods_iterator_with_riven {
            iter.next().map(|((status_mods, other_mods), riven_mod)| {
                let mut build: Vec<T> = vec![];
                if status_mods.len() > 0 {
                    build.push(status_mods[0].clone());
                }
                build.push(riven_mod.clone());
                if status_mods.len() > 0 {
                    build.extend(status_mods.iter().skip(1).cloned().cloned());
                }
                build.extend(other_mods.iter().cloned().cloned());
                build.extend(self.obligatory_mods.iter().cloned());
                build
            })
        } else if let Some(iter) = &mut self.mods_iterator_without_riven {
            iter.next().map(|(status_mods, other_mods)| {
                let mut build: Vec<T> = vec![];
                build.extend(status_mods.iter().cloned().cloned());
                build.extend(other_mods.iter().cloned().cloned());
                build.extend(self.obligatory_mods.iter().cloned());
                build
            })
        } else {
            None
        }
    }
}
