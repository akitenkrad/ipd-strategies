//! 行動の定数と，戦略の状態を入れる実験状態の型を定める．

use serde::{Deserialize, Serialize};
use socsim_core::AgentId;
use socsim_games::{
    AutomatonPolicyState, BeliefState, BuiltinPolicyState, DecisionContext, EstimateState,
    ExperimentState, MatrixGame, PropensityState, Slot, Stateless, TftState,
};

pub const C: usize = 0; // 行動 0 = 協力
pub const D: usize = 1; // 行動 1 = 裏切り

/// エージェントごとの戦略の状態．組込みの戦略と状態を持つ自作の戦略を収める．
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Policy {
    Builtin(BuiltinPolicyState),
    Alternator {
        next: usize,
    },
    Gradual {
        remaining_defections: usize,
        remaining_calming: usize,
    },
}

/// 組込みの戦略（TFT や wsls）が自分の状態を取り出せるよう，`Builtin` に委ねる．
macro_rules! builtin_slot {
    ($t:ty) => {
        impl Slot<$t> for Policy {
            fn slot(&self) -> Option<&$t> {
                match self {
                    Policy::Builtin(s) => s.slot(),
                    _ => None,
                }
            }
            fn slot_mut(&mut self) -> Option<&mut $t> {
                match self {
                    Policy::Builtin(s) => s.slot_mut(),
                    _ => None,
                }
            }
            fn from_slot(value: $t) -> Self {
                Policy::Builtin(<BuiltinPolicyState as Slot<$t>>::from_slot(value))
            }
        }
    };
}
builtin_slot!(Stateless);
builtin_slot!(TftState);
builtin_slot!(AutomatonPolicyState);
builtin_slot!(BeliefState);
builtin_slot!(EstimateState);
builtin_slot!(PropensityState);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exp;

impl ExperimentState for Exp {
    type PolicyState = Policy;
    type View<'a> = ();
    fn view_for(&self, _me: AgentId) -> Self::View<'_> {}
}

pub type Ctx<'a> = DecisionContext<'a, MatrixGame, Exp>;
