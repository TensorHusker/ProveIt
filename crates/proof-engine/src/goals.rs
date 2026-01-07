//! Goal management and proof state tracking

use im::Vector;
use sctt_core::{Context, Expr, Value};
use serde::{Deserialize, Serialize}; // Persistent vector for efficient undo/redo

use crate::{Error, Result};

/// A proof goal to be satisfied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Unique identifier
    pub id: GoalId,
    /// The proposition to prove
    pub proposition: Expr,
    /// Hypotheses available for this goal
    pub hypotheses: Vec<Hypothesis>,
    /// Optional name for the goal
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GoalId(pub u64);

/// A hypothesis in the context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    /// Name of the hypothesis
    pub name: String,
    /// Type of the hypothesis
    pub ty: Expr,
    /// Optional body (for definitions)
    pub body: Option<Expr>,
}

impl Goal {
    pub fn new(id: GoalId, proposition: Expr) -> Self {
        Self {
            id,
            proposition,
            hypotheses: Vec::new(),
            name: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_hypothesis(&mut self, hypothesis: Hypothesis) {
        self.hypotheses.push(hypothesis);
    }

    /// Convert hypotheses to a type checking context
    pub fn to_context(&self) -> Context {
        let mut ctx = Context::new();
        for hyp in &self.hypotheses {
            // Evaluate the type
            let ty_value = sctt_core::eval(&hyp.ty, &sctt_core::Env::new());
            ctx = ctx.extend(sctt_core::Name(hyp.name.clone()), ty_value);
        }
        ctx
    }
}

impl Hypothesis {
    pub fn new(name: String, ty: Expr) -> Self {
        Self {
            name,
            ty,
            body: None,
        }
    }

    pub fn with_body(mut self, body: Expr) -> Self {
        self.body = Some(body);
        self
    }
}

/// State of an interactive proof
#[derive(Debug, Clone)]
pub struct ProofState {
    /// Current goals to prove
    goals: Vector<Goal>,
    /// Completed goals
    completed: Vec<Goal>,
    /// History for undo/redo
    history: Vector<ProofSnapshot>,
    /// Current history position
    history_pos: usize,
    /// Next goal ID
    next_goal_id: u64,
}

/// Snapshot of proof state for undo/redo
#[derive(Debug, Clone)]
struct ProofSnapshot {
    goals: Vector<Goal>,
    completed: Vec<Goal>,
}

impl ProofState {
    pub fn new() -> Self {
        Self {
            goals: Vector::new(),
            completed: Vec::new(),
            history: Vector::new(),
            history_pos: 0,
            next_goal_id: 0,
        }
    }

    /// Create a proof state with an initial goal
    pub fn with_goal(proposition: Expr) -> Self {
        let mut state = Self::new();
        state.add_goal(Goal::new(GoalId(0), proposition));
        state.next_goal_id = 1;
        state
    }

    /// Add a new goal
    pub fn add_goal(&mut self, goal: Goal) {
        self.save_snapshot();
        self.goals.push_back(goal);
    }

    /// Get the current goal (first in the list)
    pub fn current_goal(&self) -> Option<&Goal> {
        self.goals.front()
    }

    /// Get a mutable reference to the current goal
    pub fn current_goal_mut(&mut self) -> Option<&mut Goal> {
        self.goals.front_mut()
    }

    /// Get a goal by ID
    pub fn get_goal(&self, id: GoalId) -> Option<&Goal> {
        self.goals.iter().find(|g| g.id == id)
    }

    /// Remove the current goal (when solved)
    pub fn complete_current_goal(&mut self) -> Result<Goal> {
        self.save_snapshot();
        let goal = self.goals.pop_front().ok_or(Error::GoalNotFound)?;
        self.completed.push(goal.clone());
        Ok(goal)
    }

    /// Replace the current goal with new subgoals
    pub fn replace_goal(&mut self, subgoals: Vec<Goal>) {
        self.save_snapshot();
        self.goals.pop_front();
        for goal in subgoals.into_iter().rev() {
            self.goals.push_front(goal);
        }
    }

    /// Get all current goals
    pub fn goals(&self) -> impl Iterator<Item = &Goal> {
        self.goals.iter()
    }

    /// Get completed goals
    pub fn completed_goals(&self) -> &[Goal] {
        &self.completed
    }

    /// Check if all goals are completed
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty()
    }

    /// Get the number of remaining goals
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }

    /// Generate a new goal ID
    pub fn new_goal_id(&mut self) -> GoalId {
        let id = GoalId(self.next_goal_id);
        self.next_goal_id += 1;
        id
    }

    /// Save current state to history
    fn save_snapshot(&mut self) {
        // Truncate future history if we're not at the end
        if self.history_pos < self.history.len() {
            self.history = self.history.clone().take(self.history_pos);
        }

        let snapshot = ProofSnapshot {
            goals: self.goals.clone(),
            completed: self.completed.clone(),
        };

        self.history.push_back(snapshot);
        self.history_pos = self.history.len();
    }

    /// Undo the last operation
    pub fn undo(&mut self) -> Result<()> {
        if self.history_pos > 0 {
            self.history_pos -= 1;
            let snapshot = &self.history[self.history_pos];
            self.goals = snapshot.goals.clone();
            self.completed = snapshot.completed.clone();
            Ok(())
        } else {
            Err(Error::InvalidProofState("No history to undo".to_string()))
        }
    }

    /// Redo the last undone operation
    pub fn redo(&mut self) -> Result<()> {
        if self.history_pos < self.history.len() {
            let snapshot = &self.history[self.history_pos];
            self.goals = snapshot.goals.clone();
            self.completed = snapshot.completed.clone();
            self.history_pos += 1;
            Ok(())
        } else {
            Err(Error::InvalidProofState("No history to redo".to_string()))
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history_pos > 0
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history_pos < self.history.len()
    }

    /// Get a summary of the proof state
    pub fn summary(&self) -> ProofStateSummary {
        ProofStateSummary {
            num_goals: self.goals.len(),
            num_completed: self.completed.len(),
            current_goal: self.current_goal().map(|g| g.proposition.clone()),
            can_undo: self.can_undo(),
            can_redo: self.can_redo(),
        }
    }
}

impl Default for ProofState {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of proof state for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofStateSummary {
    pub num_goals: usize,
    pub num_completed: usize,
    pub current_goal: Option<Expr>,
    pub can_undo: bool,
    pub can_redo: bool,
}

/// Goal state for a specific goal
#[derive(Debug, Clone)]
pub struct GoalState {
    /// The goal itself
    pub goal: Goal,
    /// Current context
    pub context: Context,
    /// Partial proof term (if any)
    pub partial_proof: Option<Expr>,
}

impl GoalState {
    pub fn new(goal: Goal) -> Self {
        let context = goal.to_context();
        Self {
            goal,
            context,
            partial_proof: None,
        }
    }

    /// Add a hypothesis to the goal
    pub fn add_hypothesis(&mut self, hyp: Hypothesis) {
        self.goal.add_hypothesis(hyp.clone());
        // Update context
        let ty_value = sctt_core::eval(&hyp.ty, &sctt_core::Env::new());
        self.context = self.context.extend(sctt_core::Name(hyp.name), ty_value);
    }

    /// Set the partial proof
    pub fn set_partial_proof(&mut self, proof: Expr) {
        self.partial_proof = Some(proof);
    }

    /// Check if the goal is solved by a proof term
    pub fn check_solved(&self, proof: &Expr) -> Result<bool> {
        let expected_ty = sctt_core::eval(&self.goal.proposition, self.context.env());
        match sctt_core::check_type(&self.context, proof, &expected_ty) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sctt_core::Name;

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new(GoalId(0), Expr::Type(0));
        assert_eq!(goal.id, GoalId(0));
        assert_eq!(goal.hypotheses.len(), 0);
    }

    #[test]
    fn test_hypothesis() {
        let hyp = Hypothesis::new("x".to_string(), Expr::Type(0));
        assert_eq!(hyp.name, "x");
        assert!(hyp.body.is_none());
    }

    #[test]
    fn test_proof_state() {
        let mut state = ProofState::new();
        let goal = Goal::new(GoalId(0), Expr::Type(0));
        state.add_goal(goal);

        assert_eq!(state.num_goals(), 1);
        assert!(!state.is_complete());
    }

    #[test]
    fn test_proof_state_completion() {
        let mut state = ProofState::with_goal(Expr::Type(0));
        assert_eq!(state.num_goals(), 1);

        state.complete_current_goal().unwrap();
        assert_eq!(state.num_goals(), 0);
        assert!(state.is_complete());
    }

    #[test]
    fn test_undo_redo() {
        let mut state = ProofState::with_goal(Expr::Type(0));
        let initial_goals = state.num_goals();

        state.add_goal(Goal::new(GoalId(1), Expr::Type(1)));
        assert_eq!(state.num_goals(), initial_goals + 1);

        state.undo().unwrap();
        assert_eq!(state.num_goals(), initial_goals);

        state.redo().unwrap();
        assert_eq!(state.num_goals(), initial_goals + 1);
    }

    #[test]
    fn test_goal_state() {
        let goal = Goal::new(GoalId(0), Expr::Type(0));
        let goal_state = GoalState::new(goal);

        assert!(goal_state.partial_proof.is_none());
    }
}
