// src/components/ImageNode.tsx
import React from 'react';
import { NodeProps, Handle, Position } from '@xyflow/react';

const ImageNode: React.FC<NodeProps> = ({ data }: any) => {
  
  return (
    <div className="p-5 bg-white border rounded-lg flex flex-col items-center shadow-md cursor-pointer">
      <img src={data.image} alt="Node Icon" className="w-16 h-16 mb-2 pointer-events-none" />
      <div>{data.label}</div>
      <Handle type="source" position={Position.Left} />
      <Handle type="target" position={Position.Right} />
    </div>
  );
};

export default ImageNode;
