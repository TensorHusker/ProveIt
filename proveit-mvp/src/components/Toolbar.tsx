interface ToolbarProps {
  onAddAssumption: () => void;
  onAddRule: () => void;
  onAddGoal: () => void;
  onCheckProof: () => void;
  onClearAll: () => void;
  onExportJson: () => void;
}

export function Toolbar({
  onAddAssumption,
  onAddRule,
  onAddGoal,
  onCheckProof,
  onClearAll,
  onExportJson,
}: ToolbarProps) {
  return (
    <div className="fixed top-4 left-4 bg-white rounded-lg shadow-lg p-3 space-y-2 z-10">
      <div className="text-sm font-bold text-gray-700 mb-2">ProveIt MVP</div>

      <button
        onClick={onAddAssumption}
        className="w-full px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-md text-sm font-medium transition-colors flex items-center justify-center gap-2"
      >
        <span>📌</span>
        Add Assumption
      </button>

      <button
        onClick={onAddRule}
        className="w-full px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded-md text-sm font-medium transition-colors flex items-center justify-center gap-2"
      >
        <span>⚡</span>
        Add Rule
      </button>

      <button
        onClick={onAddGoal}
        className="w-full px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white rounded-md text-sm font-medium transition-colors flex items-center justify-center gap-2"
      >
        <span>🎯</span>
        Add Goal
      </button>

      <div className="border-t border-gray-200 my-2"></div>

      <button
        onClick={onCheckProof}
        className="w-full px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-md text-sm font-medium transition-colors"
      >
        Check Proof
      </button>

      <button
        onClick={onClearAll}
        className="w-full px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-md text-sm font-medium transition-colors"
      >
        Clear All
      </button>

      <button
        onClick={onExportJson}
        className="w-full px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-md text-sm font-medium transition-colors"
      >
        Export JSON
      </button>
    </div>
  );
}
