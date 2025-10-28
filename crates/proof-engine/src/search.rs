//! Automated proof search

use sctt_core::{Context, Expr};
use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

use crate::goals::{Goal, GoalState};
use crate::tactics::{Tactic, TacticLibrary, TacticResult};
use crate::{Error, Result};

/// Automated proof search engine
pub struct ProofSearch {
    /// Available tactics
    tactics: TacticLibrary,
    /// Search strategy
    strategy: SearchStrategy,
    /// Maximum search depth
    max_depth: usize,
    /// Maximum search time
    max_time: Duration,
}

/// Strategy for proof search
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchStrategy {
    /// Breadth-first search
    BreadthFirst,
    /// Depth-first search
    DepthFirst,
    /// Iterative deepening
    IterativeDeepening,
    /// Best-first search (using heuristics)
    BestFirst,
}

/// A node in the search tree
#[derive(Debug, Clone)]
struct SearchNode {
    /// Current goals
    goals: Vec<Goal>,
    /// Proof terms found so far
    proofs: Vec<Expr>,
    /// Depth in search tree
    depth: usize,
    /// Parent node (for backtracking)
    parent: Option<Box<SearchNode>>,
    /// Tactic that led to this node
    tactic_used: Option<String>,
}

/// Result of a proof search
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Whether a proof was found
    pub success: bool,
    /// The proof term (if found)
    pub proof: Option<Expr>,
    /// Number of nodes explored
    pub nodes_explored: usize,
    /// Time taken
    pub time_taken: Duration,
    /// Maximum depth reached
    pub max_depth_reached: usize,
    /// Tactics used (in order)
    pub tactic_sequence: Vec<String>,
}

impl ProofSearch {
    pub fn new() -> Self {
        Self {
            tactics: TacticLibrary::new(),
            strategy: SearchStrategy::IterativeDeepening,
            max_depth: 10,
            max_time: Duration::from_secs(5),
        }
    }

    pub fn with_strategy(mut self, strategy: SearchStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn with_max_time(mut self, duration: Duration) -> Self {
        self.max_time = duration;
        self
    }

    /// Search for a proof of the given goal
    pub fn search(&self, goal: Goal) -> Result<SearchResult> {
        let start_time = Instant::now();

        match self.strategy {
            SearchStrategy::BreadthFirst => self.breadth_first_search(goal, start_time),
            SearchStrategy::DepthFirst => self.depth_first_search(goal, start_time),
            SearchStrategy::IterativeDeepening => self.iterative_deepening_search(goal, start_time),
            SearchStrategy::BestFirst => self.best_first_search(goal, start_time),
        }
    }

    /// Breadth-first search
    fn breadth_first_search(&self, goal: Goal, start_time: Instant) -> Result<SearchResult> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut nodes_explored = 0;
        let mut max_depth_reached = 0;

        let initial_node = SearchNode {
            goals: vec![goal],
            proofs: vec![],
            depth: 0,
            parent: None,
            tactic_used: None,
        };

        queue.push_back(initial_node);

        while let Some(node) = queue.pop_front() {
            nodes_explored += 1;
            max_depth_reached = max_depth_reached.max(node.depth);

            // Check timeout
            if start_time.elapsed() > self.max_time {
                return Ok(SearchResult {
                    success: false,
                    proof: None,
                    nodes_explored,
                    time_taken: start_time.elapsed(),
                    max_depth_reached,
                    tactic_sequence: vec![],
                });
            }

            // Check if we've solved all goals
            if node.goals.is_empty() {
                let tactic_sequence = self.extract_tactic_sequence(&node);
                return Ok(SearchResult {
                    success: true,
                    proof: node.proofs.first().cloned(),
                    nodes_explored,
                    time_taken: start_time.elapsed(),
                    max_depth_reached,
                    tactic_sequence,
                });
            }

            // Check depth limit
            if node.depth >= self.max_depth {
                continue;
            }

            // Try each tactic on the first goal
            if let Some(current_goal) = node.goals.first() {
                let goal_state = GoalState::new(current_goal.clone());

                for tactic_name in self.tactics.list() {
                    if let Some(tactic) = self.tactics.get(tactic_name) {
                        match tactic.apply(&goal_state) {
                            TacticResult::Success { subgoals, .. } => {
                                // Create new node with subgoals
                                let mut new_goals = subgoals;
                                new_goals.extend_from_slice(&node.goals[1..]);

                                let new_node = SearchNode {
                                    goals: new_goals,
                                    proofs: node.proofs.clone(),
                                    depth: node.depth + 1,
                                    parent: Some(Box::new(node.clone())),
                                    tactic_used: Some(tactic_name.to_string()),
                                };

                                // Use a simple hash to avoid revisiting
                                let state_hash = format!("{:?}", new_node.goals);
                                if !visited.contains(&state_hash) {
                                    visited.insert(state_hash);
                                    queue.push_back(new_node);
                                }
                            }
                            TacticResult::Failure(_) => {
                                // Tactic failed, try next one
                            }
                        }
                    }
                }
            }
        }

        Ok(SearchResult {
            success: false,
            proof: None,
            nodes_explored,
            time_taken: start_time.elapsed(),
            max_depth_reached,
            tactic_sequence: vec![],
        })
    }

    /// Depth-first search
    fn depth_first_search(&self, goal: Goal, start_time: Instant) -> Result<SearchResult> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        let mut nodes_explored = 0;
        let mut max_depth_reached = 0;

        let initial_node = SearchNode {
            goals: vec![goal],
            proofs: vec![],
            depth: 0,
            parent: None,
            tactic_used: None,
        };

        stack.push(initial_node);

        while let Some(node) = stack.pop() {
            nodes_explored += 1;
            max_depth_reached = max_depth_reached.max(node.depth);

            if start_time.elapsed() > self.max_time {
                return Ok(SearchResult {
                    success: false,
                    proof: None,
                    nodes_explored,
                    time_taken: start_time.elapsed(),
                    max_depth_reached,
                    tactic_sequence: vec![],
                });
            }

            if node.goals.is_empty() {
                let tactic_sequence = self.extract_tactic_sequence(&node);
                return Ok(SearchResult {
                    success: true,
                    proof: node.proofs.first().cloned(),
                    nodes_explored,
                    time_taken: start_time.elapsed(),
                    max_depth_reached,
                    tactic_sequence,
                });
            }

            if node.depth >= self.max_depth {
                continue;
            }

            if let Some(current_goal) = node.goals.first() {
                let goal_state = GoalState::new(current_goal.clone());

                for tactic_name in self.tactics.list() {
                    if let Some(tactic) = self.tactics.get(tactic_name) {
                        match tactic.apply(&goal_state) {
                            TacticResult::Success { subgoals, .. } => {
                                let mut new_goals = subgoals;
                                new_goals.extend_from_slice(&node.goals[1..]);

                                let new_node = SearchNode {
                                    goals: new_goals,
                                    proofs: node.proofs.clone(),
                                    depth: node.depth + 1,
                                    parent: Some(Box::new(node.clone())),
                                    tactic_used: Some(tactic_name.to_string()),
                                };

                                let state_hash = format!("{:?}", new_node.goals);
                                if !visited.contains(&state_hash) {
                                    visited.insert(state_hash);
                                    stack.push(new_node);
                                }
                            }
                            TacticResult::Failure(_) => {}
                        }
                    }
                }
            }
        }

        Ok(SearchResult {
            success: false,
            proof: None,
            nodes_explored,
            time_taken: start_time.elapsed(),
            max_depth_reached,
            tactic_sequence: vec![],
        })
    }

    /// Iterative deepening search
    fn iterative_deepening_search(&self, goal: Goal, start_time: Instant) -> Result<SearchResult> {
        for depth_limit in 1..=self.max_depth {
            let mut searcher = self.clone();
            searcher.max_depth = depth_limit;

            match searcher.depth_first_search(goal.clone(), start_time) {
                Ok(result) => {
                    if result.success {
                        return Ok(result);
                    }
                    // Continue to next depth if time allows
                    if start_time.elapsed() > self.max_time {
                        return Ok(result);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(SearchResult {
            success: false,
            proof: None,
            nodes_explored: 0,
            time_taken: start_time.elapsed(),
            max_depth_reached: self.max_depth,
            tactic_sequence: vec![],
        })
    }

    /// Best-first search (using simple heuristics)
    fn best_first_search(&self, goal: Goal, start_time: Instant) -> Result<SearchResult> {
        // For now, just use BFS as a placeholder
        // A real implementation would use a priority queue with heuristics
        self.breadth_first_search(goal, start_time)
    }

    /// Extract the sequence of tactics used from a successful node
    fn extract_tactic_sequence(&self, node: &SearchNode) -> Vec<String> {
        let mut sequence = Vec::new();
        let mut current = node;

        while let Some(tactic_name) = &current.tactic_used {
            sequence.push(tactic_name.clone());
            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                break;
            }
        }

        sequence.reverse();
        sequence
    }

    /// Clone for iterative deepening
    fn clone(&self) -> Self {
        Self {
            tactics: TacticLibrary::new(), // Create fresh library
            strategy: self.strategy,
            max_depth: self.max_depth,
            max_time: self.max_time,
        }
    }
}

impl Default for ProofSearch {
    fn default() -> Self {
        Self::new()
    }
}

/// Heuristic evaluation for goals (lower is better)
fn evaluate_goal(goal: &Goal) -> usize {
    // Simple heuristic: count the size of the proposition
    count_expr_size(&goal.proposition)
}

fn count_expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::Var(_, _) | Expr::Type(_) => 1,
        Expr::Pi {
            domain, codomain, ..
        } => 1 + count_expr_size(domain) + count_expr_size(codomain),
        Expr::Lambda { body, .. } => 1 + count_expr_size(body),
        Expr::App { func, arg } => 1 + count_expr_size(func) + count_expr_size(arg),
        Expr::Path { ty, left, right } => {
            1 + count_expr_size(ty) + count_expr_size(left) + count_expr_size(right)
        }
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goals::GoalId;

    #[test]
    fn test_proof_search_creation() {
        let search = ProofSearch::new();
        assert_eq!(search.strategy, SearchStrategy::IterativeDeepening);
    }

    #[test]
    fn test_search_strategies() {
        let search = ProofSearch::new()
            .with_strategy(SearchStrategy::BreadthFirst)
            .with_max_depth(5)
            .with_max_time(Duration::from_secs(1));

        assert_eq!(search.strategy, SearchStrategy::BreadthFirst);
        assert_eq!(search.max_depth, 5);
    }

    #[test]
    fn test_goal_evaluation() {
        let goal = Goal::new(GoalId(0), Expr::Type(0));
        let size = evaluate_goal(&goal);
        assert_eq!(size, 1);
    }

    #[test]
    fn test_search_result() {
        let result = SearchResult {
            success: true,
            proof: Some(Expr::Type(0)),
            nodes_explored: 10,
            time_taken: Duration::from_millis(100),
            max_depth_reached: 3,
            tactic_sequence: vec!["intro".to_string()],
        };

        assert!(result.success);
        assert_eq!(result.nodes_explored, 10);
        assert_eq!(result.tactic_sequence.len(), 1);
    }
}
