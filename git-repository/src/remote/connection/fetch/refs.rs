use crate::remote::fetch::RefMap;

///
pub mod update {
    mod error {
        /// The error returned when updating refs after a fetch operation.
        #[derive(Debug, thiserror::Error)]
        #[error("TBD")]
        pub struct Error {}
    }
    use crate::remote::fetch::RefMap;
    pub use error::Error;

    /// The outcome of the refs-update operation at the end of a fetch.
    #[derive(Debug, Clone)]
    pub struct Outcome {
        /// All edits that were performed to update local refs.
        pub edits: Vec<git_ref::transaction::RefEdit>,
        /// Each update provides more information about what happened to the corresponding mapping.
        /// Use [`iter_mapping_updates()`][Self::iter_mapping_updates()] to recombine the update information with ref-edits and their
        /// mapping.
        pub updates: Vec<super::Update>,
    }

    /// Describe the way a ref was updated
    #[derive(Debug, Copy, Clone)]
    pub enum Mode {
        /// The old ref's commit was an ancestor of the new one, allowing for a fast-forward without a merge.
        FastForward,
        /// The ref was set to point to the new commit from the remote without taking into consideration its ancestry.
        Forced,
        /// No change was attempted as the remote ref didn't change compared to the current ref, or because no remote ref was specified
        /// in the ref-spec.
        NoChangeNeeded,
    }

    impl Outcome {
        /// Produce an iterator over all information used to produce the this outcome, ref-update by ref-update, using the `ref_map`
        /// used when producing the ref update.
        pub fn iter_mapping_updates<'a>(
            &self,
            ref_map: &'a RefMap<'_>,
        ) -> impl Iterator<
            Item = (
                &super::Update,
                &'a crate::remote::fetch::Mapping,
                Option<&git_ref::transaction::RefEdit>,
            ),
        > {
            self.updates
                .iter()
                .zip(ref_map.mappings.iter())
                .map(move |(update, mapping)| (update, mapping, update.edit_index.and_then(|idx| self.edits.get(idx))))
        }
    }
}
use git_refspec::RefSpec;

/// Information about the update of a single reference, corresponding the respective entry in [`RefMap::mapping`].
#[derive(Debug, Clone, Copy)]
pub struct Update {
    /// The way the update was performed.
    pub mode: update::Mode,
    /// The index to the edit that was created from the corresponding mapping, or `None` if there was no local ref.
    pub edit_index: Option<usize>,
}

pub(crate) fn update(
    _repo: &crate::Repository,
    _remote: &[RefSpec],
    _ref_map: &RefMap<'_>,
) -> Result<update::Outcome, update::Error> {
    // TODO: tests and impl
    Ok(update::Outcome {
        edits: Default::default(),
        updates: Default::default(),
    })
}
