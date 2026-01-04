import { Node } from 'reactflow';

export interface NodeData {
  label: string;
  proposition?: string;
  rule?: string;
  validated?: boolean;
  inputs?: number;
  outputs?: number;
}

export type NodeType = 'assumptionNode' | 'ruleNode' | 'goalNode';

export interface CustomNode extends Node {
  type: NodeType;
  data: NodeData;
}
