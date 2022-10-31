// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod bytes;
mod serialize;
mod string;

use crate::Transition;
use console::{account::Field, network::prelude::*};

use indexmap::IndexMap;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Execution<N: Network> {
    /// The edition.
    edition: u16,
    /// The transitions.
    transitions: IndexMap<N::TransitionID, Transition<N>>,
}

impl<N: Network> Execution<N> {
    /// Initialize a new `Execution` instance.
    pub fn new() -> Self {
        Self { edition: N::EDITION, transitions: Default::default() }
    }

    /// Initializes a new `Execution` instance with the given transitions.
    pub fn from(edition: u16, transitions: Vec<Transition<N>>) -> Result<Self> {
        // Ensure the transitions is not empty.
        ensure!(!transitions.is_empty(), "Execution cannot initialize from empty list of transitions");
        // Return the new `Execution` instance.
        match edition == N::EDITION {
            true => Ok(Self { edition, transitions: transitions.into_iter().map(|t| (*t.id(), t)).collect() }),
            false => bail!("Execution cannot initialize with a different edition"),
        }
    }

    /// Returns the edition.
    pub const fn edition(&self) -> u16 {
        self.edition
    }
}

impl<N: Network> Execution<N> {
    /// Returns `true` if the execution contains the transition for the given transition ID.
    pub fn contains_transition(&self, transition_id: &N::TransitionID) -> bool {
        self.transitions.contains_key(transition_id)
    }

    /// Returns the `Transition` corresponding to the given transition ID.
    pub fn find(&self, id: &N::TransitionID) -> Option<&Transition<N>> {
        self.transitions.get(id)
    }

    /// Returns the `Transition` at the given index.
    pub fn get(&self, index: usize) -> Result<&Transition<N>> {
        match self.transitions.get_index(index) {
            Some((_, transition)) => Ok(transition),
            None => bail!("Transition index {index} out of bounds in the execution object"),
        }
    }

    /// Returns the next `Transition` in the execution.
    pub fn peek(&self) -> Result<&Transition<N>> {
        self.get(self.len() - 1)
    }

    /// Appends the given `Transition` to the execution.
    pub fn push(&mut self, transition: Transition<N>) {
        self.transitions.insert(*transition.id(), transition);
    }

    /// Pops the last `Transition` from the execution.
    pub fn pop(&mut self) -> Result<Transition<N>> {
        match self.transitions.pop() {
            Some((_, transition)) => Ok(transition),
            None => bail!("Cannot pop a transition from an empty execution object"),
        }
    }

    /// Returns the number of transitions in the execution.
    pub fn len(&self) -> usize {
        self.transitions.len()
    }

    /// Returns `true` if the execution is empty.
    pub fn is_empty(&self) -> bool {
        self.transitions.is_empty()
    }
}

impl<N: Network> Execution<N> {
    /// Returns a consuming iterator over the underlying transitions.
    pub fn into_transitions(self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = Transition<N>> {
        self.transitions.into_values()
    }

    /// Returns an iterator over the underlying transitions.
    pub fn transitions(&self) -> impl '_ + ExactSizeIterator + DoubleEndedIterator<Item = &Transition<N>> {
        self.transitions.values()
    }

    /// Returns an iterator over the commitments.
    pub fn commitments(&self) -> impl '_ + Iterator<Item = &Field<N>> {
        self.transitions.values().flat_map(Transition::commitments)
    }
}

impl<N: Network> Deref for Execution<N> {
    type Target = IndexMap<N::TransitionID, Transition<N>>;

    fn deref(&self) -> &Self::Target {
        &self.transitions
    }
}
