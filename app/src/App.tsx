import {
	type Edge,
	type Node,
	ReactFlow,
	addEdge,
	useEdgesState,
	useNodesState,
} from "@xyflow/react";
import React, { useCallback } from "react";

import "@xyflow/react/dist/style.css";

const initialNodes: Node[] = [
	{ id: "1", position: { x: 0, y: 0 }, data: { label: "1" } },
	{ id: "2", position: { x: 0, y: 100 }, data: { label: "2" } },
];

const initialEdges: Edge[] = [{ id: "e1-2", source: "1", target: "2" }];

export default function App() {
	const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
	const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

	const onConnect = useCallback(
		(params: any) => setEdges((eds) => addEdge(params, eds)),
		[setEdges],
	);

	return (
		<div style={{ width: "100vw", height: "100vh" }}>
			<ReactFlow
				nodes={nodes}
				edges={edges}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onConnect={onConnect}
			/>
		</div>
	);
}
