import { Handle, Position } from 'reactflow';
import { useState } from 'react';
import { NodeData } from '../../types/nodes';

interface RuleNodeProps {
  data: NodeData;
  id: string;
}

export function RuleNode({ data, id }: RuleNodeProps) {
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

  const inputs = localData.inputs || 1;
  const outputs = localData.outputs || 1;

  return (
    <div
      className="px-4 py-3 rounded-lg border-2 border-green-600 bg-gradient-to-br from-green-400 to-green-500 text-white shadow-lg min-w-[140px] min-h-[80px]"
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
            value={localData.rule || ''}
            onChange={(e) => setLocalData({ ...localData, rule: e.target.value })}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            className="bg-white/20 border-none outline-none rounded px-1 w-full"
            placeholder="Rule name..."
          />
        ) : (
          localData.rule || 'Edit me'
        )}
      </div>

      {/* Input handles on the left */}
      {Array.from({ length: inputs }).map((_, index) => (
        <Handle
          key={`input-${index}`}
          type="target"
          position={Position.Left}
          id={`input-${index}`}
          style={{ top: `${((index + 1) * 100) / (inputs + 1)}%` }}
          className="w-3 h-3 bg-green-700 border-2 border-white"
        />
      ))}

      {/* Output handles on the right */}
      {Array.from({ length: outputs }).map((_, index) => (
        <Handle
          key={`output-${index}`}
          type="source"
          position={Position.Right}
          id={`output-${index}`}
          style={{ top: `${((index + 1) * 100) / (outputs + 1)}%` }}
          className="w-3 h-3 bg-green-700 border-2 border-white"
        />
      ))}
    </div>
  );
}
