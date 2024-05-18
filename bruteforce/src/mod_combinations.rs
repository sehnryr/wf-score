use std::iter::Iterator;
use std::slice::Iter;

use itertools::{Combinations, Itertools, Permutations, Product};

use wf_mods::melee::*;

pub struct ModCombinations<'a> {
    mods_iterator_with_riven: Option<
        Product<
            Product<Permutations<Iter<'a, MeleeMod>>, Combinations<Iter<'a, MeleeMod>>>,
            Iter<'a, MeleeMod>,
        >,
    >,
    mods_iterator_without_riven:
        Option<Product<Permutations<Iter<'a, MeleeMod>>, Combinations<Iter<'a, MeleeMod>>>>,
    obligatory_mods: &'a [MeleeMod],
}

impl<'a> ModCombinations<'a> {
    pub fn new(
        status_mod_count: usize,
        status_mods: &'a [MeleeMod],
        other_mods: &'a [MeleeMod],
        riven_mods: &'a [MeleeMod],
        obligatory_mods: &'a [MeleeMod],
    ) -> Self {
        let are_riven_mods = riven_mods.len() > 0;

        let status_mods_iterator = status_mods.iter().permutations(status_mod_count);
        let other_mods_iterator = other_mods.iter().combinations(
            8 - status_mod_count - obligatory_mods.len() - if are_riven_mods { 1 } else { 0 },
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

impl<'a> Iterator for ModCombinations<'a> {
    type Item = Vec<MeleeMod>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.mods_iterator_with_riven {
            iter.next().map(|((status_mods, other_mods), riven_mod)| {
                let mut build = vec![];
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
                let mut build = vec![];
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
