//! 本稿で自作する戦略．前半は状態を持たない戦略，後半は状態を持つ戦略．

use rand::Rng;
use socsim_core::SimRng;
use socsim_games::{
    BuiltinPolicyState, DecisionRule, LearnContext, MatrixGame, RuleError, RuleInitContext,
    Stateless,
};

use crate::state::{Ctx, Exp, Policy, C, D};

// ---- 状態を持たない戦略 ----

pub const GENEROSITY: f64 = 1.0 / 3.0; // Generous TFT が裏切りを許す確率
pub const JOSS_DEFECT: f64 = 0.1; // Joss が協力の代わりに裏切る確率

/// 戦略が読む履歴: 相手の過去の手（古い順）と，自分の前回の手．
pub struct Seen {
    pub opp: Vec<usize>,
    pub mine: Option<usize>,
}

pub fn seen(ctx: &Ctx<'_>) -> Result<Seen, RuleError> {
    let opponent = *ctx
        .opponents()
        .first()
        .ok_or_else(|| RuleError::Unsupported("no opponent".into()))?;
    let history = ctx
        .opponent_history(opponent)
        .ok_or_else(|| RuleError::Unsupported("no opponent history".into()))?;
    Ok(Seen {
        opp: history.actions().flatten().copied().collect(),
        mine: ctx.my_history().last().and_then(|r| r.action),
    })
}

/// 履歴と乱数から手を 1 つ選ぶ関数．
pub type Choose = fn(&Seen, &mut SimRng) -> usize;

/// 手の選び方を関数 1 つで持つだけの戦略．
pub struct HistoryRule {
    pub name: &'static str,
    pub choose: Choose,
}

impl DecisionRule<MatrixGame, Exp> for HistoryRule {
    fn name(&self) -> &str {
        self.name
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Builtin(BuiltinPolicyState::Stateless(Stateless)))
    }
    fn decide(&self, _: &mut Policy, ctx: &Ctx<'_>, rng: &mut SimRng) -> Result<usize, RuleError> {
        Ok((self.choose)(&seen(ctx)?, rng))
    }
}

/// 相手が 2 回続けて裏切ったときだけ裏切る．
pub fn tit_for_two_tats(s: &Seen, _: &mut SimRng) -> usize {
    if s.opp.ends_with(&[D, D]) {
        D
    } else {
        C
    }
}

/// 初回は裏切り，以降は相手の前回の手をまねる．
pub fn suspicious_tft(s: &Seen, _: &mut SimRng) -> usize {
    s.opp.last().copied().unwrap_or(D)
}

/// TFT と同じだが，相手の裏切りを確率 GENEROSITY で許す．
pub fn generous_tft(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        Some(&D) if !rng.gen_bool(GENEROSITY) => D,
        _ => C,
    }
}

/// TFT と同じだが，協力すべき場面で確率 JOSS_DEFECT で裏切る（初回は協力）．
pub fn joss(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        None => C,
        Some(&D) => D,
        Some(_) if rng.gen_bool(JOSS_DEFECT) => D,
        Some(_) => C,
    }
}

/// Press--Dyson の Extort-2．前回の手の組に応じた確率で協力する．
pub fn extort2(s: &Seen, rng: &mut SimRng) -> usize {
    let cooperate = match (s.mine, s.opp.last().copied()) {
        (None, _) => return C,
        (Some(C), Some(C)) => 8.0 / 9.0,
        (Some(C), Some(D)) => 1.0 / 2.0,
        (Some(D), Some(C)) => 1.0 / 3.0,
        (Some(D), Some(D)) => 0.0,
        _ => 0.0,
    };
    usize::from(!rng.gen_bool(cooperate))
}

/// 最初に D, C, C と探り，相手が応じなければ裏切り続け，応じれば TFT に移る．
pub fn prober(s: &Seen, _: &mut SimRng) -> usize {
    match s.opp.len() {
        0 => D,
        1 | 2 => C,
        _ if s.opp[1] == C && s.opp[2] == C => D,
        _ => s.opp.last().copied().unwrap_or(C),
    }
}

/// 相手の過去の手で協力が裏切り以上なら協力する．
pub fn soft_majority(s: &Seen, _: &mut SimRng) -> usize {
    let cooperations = s.opp.iter().filter(|&&action| action == C).count();
    let defections = s.opp.len() - cooperations;
    if cooperations >= defections {
        C
    } else {
        D
    }
}

// ---- 状態を持つ戦略 ----

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
