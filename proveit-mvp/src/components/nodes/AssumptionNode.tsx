import { Handle, Position } from 'reactflow';
import { useState } from 'react';
import { NodeData } from '../../types/nodes';

interface AssumptionNodeProps {
  data: NodeData;
  id: string;
}

export function AssumptionNode({ data, id }: AssumptionNodeProps) {
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
      className="px-4 py-3 rounded-lg border-2 border-blue-600 bg-gradient-to-br from-blue-400 to-blue-500 text-white shadow-lg min-w-[120px] min-h-[60px]"
      onDoubleClick={handleDoubleClick}
    >
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
            placeholder="Proposition..."
          />
        ) : (
          localData.proposition || 'Edit me'
        )}
      </div>

      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-blue-700 border-2 border-white"
      />
    </div>
  );
}
