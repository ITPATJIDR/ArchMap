// src/components/TopNavbar.tsx
import React, {useState} from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import {
  Node,
} from '@xyflow/react';

interface TopBarProps {
  nodes: Node[];
}

interface NodeData {
  label: string | unknown
  service_type: string | unknown
}

const TopNavbar: React.FC<TopBarProps> = ({nodes}) => {

  const [path, setPath] = useState<string | string[] | null>("");

  const handleSelectPath = async () => {
    const selected: string | string[] | null = await open({
      directory: true,
      multiple: true,
    })
    setPath(selected)
  }

  const handleCreate = async () => {
    const extract_service = (nodes.map((item: Node) => {
        const new_data: NodeData = {
            label: item.data.label,
            service_type: item.data.service_type,
        }
        return new_data
    }))

    if (path && extract_service) {
      invoke("check_service", {service: extract_service, path: path})
    }else {
      console.log("HI")
    }
  }

  return (
    <div className="fixed top-4 left-1/2 transform -translate-x-1/2 z-10 flex space-x-4 bg-white shadow-lg p-2 rounded-lg">
      <button onClick={handleSelectPath}>
        {path ? path : "Choose Folder"}
      </button>
      <button
        className="p-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
        onClick={handleCreate}
      >
        Create
      </button>
    </div>
  );
};

export default TopNavbar;
