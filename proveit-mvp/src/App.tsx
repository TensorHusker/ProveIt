import { useCallback, useMemo, useState } from 'react';
import ReactFlow, {
  Node,
  Edge,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  BackgroundVariant,
  ConnectionLineType,
} from 'reactflow';
import 'reactflow/dist/style.css';

import { Toolbar } from './components/Toolbar';
import { AssumptionNode } from './components/nodes/AssumptionNode';
import { RuleNode } from './components/nodes/RuleNode';
import { GoalNode } from './components/nodes/GoalNode';
import { CustomNode } from './types/nodes';

// Initial nodes as per spec
const initialNodes: Node[] = [
  {
    id: '1',
    type: 'assumptionNode',
    position: { x: 100, y: 100 },
    data: { label: 'Assumption 1', proposition: 'A ∧ B' },
  },
  {
    id: '2',
    type: 'ruleNode',
    position: { x: 350, y: 100 },
    data: { label: 'AND-Elim', rule: '∧-Elimination-Left', inputs: 1, outputs: 1 },
  },
  {
    id: '3',
    type: 'goalNode',
    position: { x: 600, y: 100 },
    data: { label: 'Goal', proposition: 'A', validated: false },
  },
];

const initialEdges: Edge[] = [];

function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const [isValidProof, setIsValidProof] = useState<boolean | null>(null);

  // Define custom node types
  const nodeTypes = useMemo(
    () => ({
      assumptionNode: AssumptionNode,
      ruleNode: RuleNode,
      goalNode: GoalNode,
    }),
    []
  );

  // Handle connections between nodes
  const onConnect = useCallback(
    (connection: Connection) => {
      setEdges((eds) => addEdge(connection, eds));
    },
    [setEdges]
  );

  // Validate connections (only allow valid connections)
  const isValidConnection = useCallback((connection: Connection) => {
    const sourceNode = nodes.find((n) => n.id === connection.source);
    const targetNode = nodes.find((n) => n.id === connection.target);

    if (!sourceNode || !targetNode) return false;

    // Valid connection patterns:
    // Assumption -> Rule
    // Assumption -> Goal
    // Rule -> Rule
    // Rule -> Goal
    const validConnections = [
      ['assumptionNode', 'ruleNode'],
      ['assumptionNode', 'goalNode'],
      ['ruleNode', 'ruleNode'],
      ['ruleNode', 'goalNode'],
    ];

    return validConnections.some(
      ([source, target]) => sourceNode.type === source && targetNode.type === target
    );
  }, [nodes]);

  // Add a new assumption node at center
  const addAssumption = useCallback(() => {
    const newNode: CustomNode = {
      id: crypto.randomUUID(),
      type: 'assumptionNode',
      position: { x: window.innerWidth / 2 - 60, y: window.innerHeight / 2 - 30 },
      data: { label: 'Assumption', proposition: 'Edit me' },
    };
    setNodes((nds) => [...nds, newNode]);
  }, [setNodes]);

  // Add a new rule node at center
  const addRule = useCallback(() => {
    const newNode: CustomNode = {
      id: crypto.randomUUID(),
      type: 'ruleNode',
      position: { x: window.innerWidth / 2 - 70, y: window.innerHeight / 2 - 40 },
      data: { label: 'Rule', rule: 'Edit me', inputs: 1, outputs: 1 },
    };
    setNodes((nds) => [...nds, newNode]);
  }, [setNodes]);

  // Add a new goal node at center
  const addGoal = useCallback(() => {
    const newNode: CustomNode = {
      id: crypto.randomUUID(),
      type: 'goalNode',
      position: { x: window.innerWidth / 2 - 60, y: window.innerHeight / 2 - 30 },
      data: { label: 'Goal', proposition: 'Edit me', validated: false },
    };
    setNodes((nds) => [...nds, newNode]);
  }, [setNodes]);

  // Check proof (placeholder for now)
  const checkProof = useCallback(() => {
    console.log('Check Proof clicked - validation logic to be implemented');
    alert('Proof validation will be implemented in Phase 2!');
  }, []);

  // Clear all nodes and edges
  const clearAll = useCallback(() => {
    if (window.confirm('Are you sure you want to clear all nodes?')) {
      setNodes([]);
      setEdges([]);
    }
  }, [setNodes, setEdges]);

  // Export to JSON
  const exportJson = useCallback(() => {
    const data = {
      nodes: nodes,
      edges: edges,
      timestamp: new Date().toISOString(),
    };
    const json = JSON.stringify(data, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `proof-${Date.now()}.json`;
    link.click();
    URL.revokeObjectURL(url);
  }, [nodes, edges]);

  return (
    <div className="w-screen h-screen">
      <Toolbar
        onAddAssumption={addAssumption}
        onAddRule={addRule}
        onAddGoal={addGoal}
        onCheckProof={checkProof}
        onClearAll={clearAll}
        onExportJson={exportJson}
      />
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        isValidConnection={isValidConnection}
        nodeTypes={nodeTypes}
        connectionLineType={ConnectionLineType.SmoothStep}
        connectionLineStyle={{ stroke: '#888', strokeWidth: 2 }}
        defaultViewport={{ x: 0, y: 0, zoom: 1 }}
        minZoom={0.5}
        maxZoom={2}
        fitView
      >
        <Background variant={BackgroundVariant.Dots} gap={20} size={1} color="#e5e7eb" />
        <Controls />
      </ReactFlow>
    </div>
  );
}

export default App;
