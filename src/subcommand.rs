use crate::common::*;

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Subcommand {
  Dump,
  Edit,
  Evaluate {
    overrides: BTreeMap<String, String>,
  },
  Init,
  List,
  Run {
    overrides: BTreeMap<String, String>,
    arguments: Vec<String>,
  },
  Show {
    name: String,
  },
  Summary,
}
