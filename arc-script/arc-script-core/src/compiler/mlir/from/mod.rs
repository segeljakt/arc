/// Module for converting the [`crate::repr::dfg::DFG`] into MLIR.
pub(crate) mod lower;

use crate::compiler::hir::HIR;
use crate::compiler::info::Info;
use crate::compiler::mlir::MLIR;
use arc_script_core_shared::Lower;
use arc_script_core_shared::OrdMap;

use tracing::instrument;

impl MLIR {
    #[instrument(name = "HIR & Info => MLIR", level = "debug", skip(hir, info))]
    pub(crate) fn from(hir: &HIR, info: &mut Info) -> Self {
        let ctx = &mut lower::Context::new(hir, info);
        let defs = hir
            .items
            .iter()
            .filter_map(|x| Some((*x, hir.defs.get(x).unwrap().lower(ctx)?)))
            .collect::<OrdMap<_, _>>();
        Self::new(hir.items.clone(), defs)
    }
}
