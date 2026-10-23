//! 状態を持つ規則（Alternator と Gradual）を定める．

use socsim_core::SimRng;
use socsim_games::{DecisionRule, LearnContext, MatrixGame, RuleError, RuleInitContext};

use crate::history_rules::seen;
use crate::state::{Ctx, Exp, Policy, C, D};

/// C と D を交互に出す．次の手を状態に持ち，ステップの終わりに入れ替える．
pub struct Alternator;

impl DecisionRule<MatrixGame, Exp> for Alternator {
    fn name(&self) -> &str {
        "alternator"
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Alternator { next: C })
    }
    fn decide(&self, state: &mut Policy, _: &Ctx<'_>, _: &mut SimRng) -> Result<usize, RuleError> {
        match state {
            Policy::Alternator { next } => Ok(*next),
            _ => Err(RuleError::MissingSlot("Alternator")),
        }
    }
    fn learn_step(
        &self,
        state: &mut Policy,
        _: &LearnContext<'_, MatrixGame, Exp>,
    ) -> Result<(), RuleError> {
        match state {
            Policy::Alternator { next } => {
                *next = if *next == C { D } else { C };
                Ok(())
            }
            _ => Err(RuleError::MissingSlot("Alternator")),
        }
    }
}

/// 相手の累積裏切り回数だけ報復し，2 回協力して鎮静する．
pub struct Gradual;

impl DecisionRule<MatrixGame, Exp> for Gradual {
    fn name(&self) -> &str {
        "gradual"
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Gradual {
            remaining_defections: 0,
            remaining_calming: 0,
        })
    }
    fn decide(
        &self,
        state: &mut Policy,
        ctx: &Ctx<'_>,
        _: &mut SimRng,
    ) -> Result<usize, RuleError> {
        let Policy::Gradual {
            remaining_defections,
            remaining_calming,
        } = state
        else {
            return Err(RuleError::MissingSlot("Gradual"));
        };
        if *remaining_defections > 0 {
            *remaining_defections -= 1;
            return Ok(D);
        }
        if *remaining_calming > 0 {
            *remaining_calming -= 1;
            return Ok(C);
        }

        let history = seen(ctx)?.opp;
        if history.last() == Some(&D) {
            let defections = history.iter().filter(|&&action| action == D).count();
            *remaining_defections = defections - 1;
            *remaining_calming = 2;
            Ok(D)
        } else {
            Ok(C)
        }
    }
}
