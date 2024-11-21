import {
	type Connection,
	type Edge,
	type Node,
	ReactFlow,
	addEdge,
	useEdgesState,
	useNodesState,
} from "@xyflow/react";
import { useCallback } from "react";

import "@xyflow/react/dist/style.css";
import Graph from "./assets/graph.json";

const initialNodes: Node[] = Graph.nodes.map(
	(n) =>
		({
			...n,
			position: { x: Math.random() * 500, y: Math.random() * 500 },
			data: { ...n.data, label: n.id },
		}) satisfies Node,
);

const initialEdges: Edge[] = Graph.edges;

export default function App() {
	const [nodes, _, onNodesChange] = useNodesState(initialNodes);
	const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

	const onConnect = useCallback(
		(params: Connection) => setEdges((eds) => addEdge(params, eds)),
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
