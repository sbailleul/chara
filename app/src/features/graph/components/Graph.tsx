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
import graph from "@/assets/graph.json";
import Elk from "elkjs";

const elk = new Elk({
	defaultLayoutOptions: {
		"elk.algorithm": "layered",
		"elk.direction": "DOWN",
		"elk.spacing.nodeNode": "25",
		"elk.layered.spacing.nodeNodeBetweenLayers": "50",
		"elk.layered.spacing": "50",
		"elk.layered.mergeEdges": "true",
		"elk.spacing": "50",
		"elk.spacing.individual": "50",
		"elk.edgeRouting": "SPLINES",
	},
});
const initialNodes: Node[] = graph.nodes.map(
	(n) =>
		({
			...n,
			position: { x: 0, y: 0 },
			width: 100,
			height: 50,
			data: { ...n.data, label: n.id },
		}) satisfies Node,
);

const initialEdges: Edge[] = graph.edges;

const layout = await elk.layout({
	id: "root",
	children: initialNodes,
	edges: initialEdges.map((e) => ({
		sources: [e.source],
		targets: [e.target],
		id: e.id,
	})),
});
export function Graph() {
	const [nodes, _, onNodesChange] = useNodesState(
		initialNodes.map((node) => {
			const positionedNode = layout.children?.find(
				(n) => n.id === node.id && !!n.x && !!n.y,
			);
			return positionedNode
				? {
						...node,
						position: {
							x: positionedNode.x as number,
							y: positionedNode.y as number,
						},
					}
				: node;
		}),
	);
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
