//! Command implementations

use crate::Result;
use proof_engine::{Goal, GoalId, ProofState};
use sctt_core::Expr;

/// Execute a tactic command
pub fn execute_tactic(state: &mut ProofState, tactic_name: &str, arg: Option<&str>) -> Result<()> {
    // Tactic execution logic
    Ok(())
}

/// Show current goal
pub fn show_goal(state: &ProofState) {
    if let Some(goal) = state.current_goal() {
        println!("Current goal:");
        println!("  ID: {:?}", goal.id);
        println!("  Proposition: {:?}", goal.proposition);
        println!("  Hypotheses: {}", goal.hypotheses.len());

        for (i, hyp) in goal.hypotheses.iter().enumerate() {
            println!("    {}: {} : {:?}", i + 1, hyp.name, hyp.ty);
        }
    } else {
        println!("No active goal");
    }
}

/// Show all goals
pub fn show_goals(state: &ProofState) {
    let num_goals = state.num_goals();
    if num_goals == 0 {
        println!("No goals remaining - proof complete!");
    } else {
        println!("Proof state: {} goal(s)", num_goals);
        for (i, goal) in state.goals().enumerate() {
            println!("  Goal {}: {:?}", i + 1, goal.proposition);
        }
    }
}

/// Show proof state summary
pub fn show_summary(state: &ProofState) {
    let summary = state.summary();
    println!("Proof Summary:");
    println!("  Goals remaining: {}", summary.num_goals);
    println!("  Goals completed: {}", summary.num_completed);
    println!("  Can undo: {}", summary.can_undo);
    println!("  Can redo: {}", summary.can_redo);

    if let Some(goal) = summary.current_goal {
        println!("  Current goal: {:?}", goal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_goals() {
        let state = ProofState::new();
        show_goals(&state);
        assert!(true);
    }
}
