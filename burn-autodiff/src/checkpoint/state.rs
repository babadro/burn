use std::{any::Any, collections::HashMap};

use crate::graph::NodeID;

/// In order to accept arbitrary node output in the same hashmap, we need to upcast them to any.
pub(crate) type StateContent = Box<dyn Any + Send + Sync>;

#[derive(Debug)]
/// The state contained at one node. Encapsulates the node output if precomputed,
/// or clearly asks that it needs to be recomputed from the parents.
/// Also keeps track of the number of times the state is required so it can be removed
/// from the map of states on its last use.
pub(crate) enum State {
    /// The state was not checkpointed, will need to recompute it from the node's parents
    Recompute { n_required: usize },
    /// The state was checkpointed or computed during retropropagation and can be directly accessed
    Computed {
        state_content: StateContent,
        n_required: usize,
    },
}

impl State {
    /// Returns a reference to the (not yet) downcasted node output, if checkpointed
    pub(crate) fn to_state_content(&self) -> &StateContent {
        match self {
            State::Recompute { n_required: _ } => {
                unreachable!("Can't get state content of recompute state. A child has likely been accessed before its parents.")
            }
            State::Computed {
                state_content,
                n_required: _,
            } => state_content,
        }
    }

    /// Returns a (not yet) downcasted node output, if checkpointed
    pub(crate) fn into_state_content(self) -> StateContent {
        match self {
            State::Recompute { n_required: _ } => {
                unreachable!("Can't get state content of recompute state. A child has likely been accessed before its parents.")
            }
            State::Computed {
                state_content,
                n_required: _,
            } => state_content,
        }
    }

    /// Returns the number of time the state is required
    pub(crate) fn n_required(&self) -> usize {
        match self {
            State::Recompute { n_required } => *n_required,
            State::Computed {
                state_content: _,
                n_required,
            } => *n_required,
        }
    }

    pub(crate) fn increment(&mut self) {
        match self {
            State::Recompute { n_required } => *n_required += 1,
            State::Computed {
                state_content: _,
                n_required,
            } => *n_required += 1,
        }
    }

    pub(crate) fn merge(&mut self, other: Self) {
        match other {
            State::Recompute { n_required: n } => match self {
                State::Recompute { n_required } => *n_required += n,
                State::Computed {
                    state_content,
                    n_required,
                } => panic!("Not supposed to happen"),
            },
            State::Computed {
                state_content,
                n_required: n,
            } => match self {
                State::Recompute { n_required } => panic!("Not supposed to happen"),
                State::Computed {
                    state_content,
                    n_required,
                } => *n_required += n,
            },
        }
    }
}

#[derive(new, Default, Debug)]
/// Links [NodeID]s to their current [State]
pub struct BackwardStates {
    map: HashMap<NodeID, State>,
}

impl BackwardStates {
    /// Returns the output in the [State] of the given [NodeID],
    /// and decrements the number of times this state is required.
    /// This function always gives ownership of the output, but will clone it if needed for further uses.
    pub(crate) fn get_state<T>(&mut self, node_id: &NodeID) -> T
    where
        T: Clone + Send + Sync + 'static,
    {
        // Fetch the state and decrement its number of required
        let state = self.map.remove(node_id).unwrap();
        let remaining_n_required = state.n_required() - 1;

        // Downcast the state to whatever it is supposed to be
        // If still needed after giving ownership, we copy it back to the hashmap
        if remaining_n_required > 0 {
            println!("reinserting node {:?}", node_id);
            let new_stored_state = match state {
                State::Recompute { n_required: _ } => State::Recompute {
                    n_required: remaining_n_required,
                },
                State::Computed {
                    state_content,
                    n_required: _,
                } => State::Computed {
                    state_content,
                    n_required: remaining_n_required,
                },
            };

            let downcasted = new_stored_state
                .to_state_content()
                .downcast_ref::<T>()
                .unwrap()
                .clone();

            self.insert_state(node_id.clone(), new_stored_state);

            downcasted
        } else {
            println!("NOT reinserting node {:?}", node_id);
            println!("{:?}", self.map.len());
            let downcasted = state.into_state_content().downcast::<T>().unwrap();
            *downcasted
        }
    }

    /// Returns a reference to the [State] of the given node
    /// Useful when we need [State] information without needing the underlying tensor
    pub(crate) fn get_state_ref(&self, node_id: &NodeID) -> Option<&State> {
        self.map.get(node_id)
    }

    /// Associates a [State] to its [NodeID]
    pub(crate) fn insert_state(&mut self, node_id: NodeID, state: State) {
        self.map.insert(node_id, state);
    }

    pub(crate) fn save<T>(&mut self, node_id: NodeID, saved_output: T)
    where
        T: Clone + Send + Sync + 'static,
    {
        let n_required = self.get_state_ref(&node_id).unwrap().n_required();
        self.insert_state(
            node_id,
            State::Computed {
                state_content: Box::new(saved_output),
                n_required,
            },
        );
    }

    pub(crate) fn extend(&mut self, other: Self) {
        // println!("extending");
        // println!("..");
        // println!("{:?}", self.map.keys());
        // println!("{:?}", self.map.values());
        // println!("..");
        // println!("{:?}", other.map.keys());
        // println!("{:?}", other.map.values());
        // println!("..");
        for (node_id, state) in other.map.into_iter() {
            // println!("{:?}", node_id);
            match self.map.remove(&node_id) {
                Some(mut s) => {
                    // println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                    s.merge(state);
                    self.map.insert(node_id, s);
                }
                None => {
                    self.map.insert(node_id, state);
                }
            }
        }
        // println!("-> {:?}", self.map.keys());
        // println!("-> {:?}", self.map.values());
        // println!("\n\n")
    }

    pub(crate) fn len(&self) -> usize {
        self.map.len()
    }

    pub(crate) fn get_mut(&mut self, node_id: &NodeID) -> Option<&mut State> {
        self.map.get_mut(node_id)
    }
}