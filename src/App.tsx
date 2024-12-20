// src/App.tsx
import React, { useCallback, useState } from 'react';
import Canvas from './components/Canvas';
import TopNavbar from './components/TopNavbar';
import RightNavbar from './components/RigthNavbar';
import ImageNode from './components/ImageNode'; // Import the custom node type
import {
  addEdge,
  applyNodeChanges,
  applyEdgeChanges,
  Node,
  Edge,
  OnConnect,
  OnNodesChange,
  OnEdgesChange,
  ReactFlowProvider,
} from '@xyflow/react';
import "./App.css"
import LeftBar from './components/LeftBar';
import { invoke } from '@tauri-apps/api/tauri';

const App: React.FC = () => {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  const nodeTypes = { imageNode: ImageNode };

  const onNodesChange: OnNodesChange = useCallback(
    (changes) => setNodes((nds) => applyNodeChanges(changes, nds)),
    [setNodes]
  );

  const onEdgesChange: OnEdgesChange = useCallback(
    (changes) => setEdges((eds) => applyEdgeChanges(changes, eds)),
    [setEdges]
  );

  const onConnect: OnConnect = useCallback(
    (connection) => setEdges((eds) => addEdge(connection, eds)),
    [setEdges]
  );

  const addNode = async (image: string, name:string, service: string) => {
    const result = await invoke<string>('get_docker_config_by_name', {
      serviceName: name
    });

    if (service === 'docker-compose' ) {
      const newNode: Node = {
        id: (nodes.length + 1).toString(),
        type: 'imageNode', // Set the node type to custom node type
        position: { x: Math.random() * 250, y: Math.random() * 250 },
        data: {
          label: `${name}_${nodes.length + 1}`,
          image,
          name: name,
          service_type: service,
          config: JSON.parse(result)
        }, // Pass the image URL
      };
      setNodes((nds) => [...nds, newNode]);
    }else {
      const newNode: Node = {
        id: (nodes.length + 1).toString(),
        type: 'imageNode', // Set the node type to custom node type
        position: { x: Math.random() * 250, y: Math.random() * 250 },
        data: {
          label: `${name}_${nodes.length + 1}`,
          image,
          name: name,
          service_type: service,
        }, 
      };
      setNodes((nds) => [...nds, newNode]);
    }
  };

  // Function to update node data
  const updateNodeData = (id: string, updates: Partial<Node['data']>) => {
    setNodes((nds) =>
      nds.map((node) =>
        node.id === id
          ? { ...node, data: { ...node.data, ...updates } }
          : node
      )
    );
  };

  return (
    <div className="w-screen h-screen bg-gray-100 relative">
      <ReactFlowProvider>
        <TopNavbar nodes={nodes} />
        <RightNavbar addNode={addNode} />
        <Canvas
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          nodeTypes={nodeTypes}
        />
        <LeftBar 
          nodes={nodes}
          updateNodeData={updateNodeData} 
        />
      </ReactFlowProvider>
    </div>
  );
};

export default App;
