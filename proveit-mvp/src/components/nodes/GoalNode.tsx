import { Handle, Position } from 'reactflow';
import { useState } from 'react';
import { NodeData } from '../../types/nodes';

interface GoalNodeProps {
  data: NodeData;
  id: string;
}

export function GoalNode({ data, id }: GoalNodeProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [localData, setLocalData] = useState(data);

  const handleDoubleClick = () => {
    setIsEditing(true);
  };

  const handleBlur = () => {
    setIsEditing(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      setIsEditing(false);
    }
  };

  return (
    <div
      className="px-4 py-3 rounded-lg border-2 border-orange-600 bg-gradient-to-br from-orange-400 to-orange-500 text-white shadow-lg min-w-[120px] min-h-[60px] relative"
      onDoubleClick={handleDoubleClick}
    >
      {/* Validation indicator */}
      <div className="absolute -top-2 -right-2 w-6 h-6 rounded-full border-2 border-white bg-white shadow-md flex items-center justify-center">
        {localData.validated ? (
          <span className="text-green-600 text-sm">✓</span>
        ) : (
          <span className="text-gray-400 text-sm">○</span>
        )}
      </div>

      <div className="text-xs font-semibold mb-1 opacity-80">
        {isEditing ? (
          <input
            type="text"
            value={localData.label}
            onChange={(e) => setLocalData({ ...localData, label: e.target.value })}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            className="bg-white/20 border-none outline-none rounded px-1 w-full"
            autoFocus
          />
        ) : (
          localData.label
        )}
      </div>
      <div className="text-lg font-bold">
        {isEditing ? (
          <input
            type="text"
            value={localData.proposition || ''}
            onChange={(e) => setLocalData({ ...localData, proposition: e.target.value })}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            className="bg-white/20 border-none outline-none rounded px-1 w-full"
            placeholder="Goal proposition..."
          />
        ) : (
          localData.proposition || 'Edit me'
        )}
      </div>

      <Handle
        type="target"
        position={Position.Left}
        className="w-3 h-3 bg-orange-700 border-2 border-white"
      />
    </div>
  );
}
