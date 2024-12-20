import React from "react";
import { Node } from '@xyflow/react';

interface LeftBarProps {
	nodes: Node[];
	updateNodeData: (id: string, updates: Partial<Node['data']>) => void;
}

interface DockerComposeNodeConfig  {
	image: string;
	ports: string[];
	environment: string[] | null;
	volumes: string[];
	command: string | null;
}

interface NodeData {
	label?: string;
	image?: string;
	name?: string;
	service_type?: string;
	config?: DockerComposeNodeConfig
}

const LeftBar: React.FC<LeftBarProps> = ({ nodes, updateNodeData }) => {
	return (
		<div
			className="
				w-[300px] h-[90vh] absolute top-20 right-4
				space-y-2 z-10 flex bg-white shadow-lg rounded-lg
			"
		>
			<div className="p-2 w-full ">
				<div className="flex items-center justify-center font-bold">
					Node Detail
				</div>
				<div className="w-full h-[85vh] overflow-auto">
					{nodes.map((item: Node, index: number) => {
						const id: string = item.id
						const data = item.data as NodeData;
						console.log(data)
						return (
							<div
								key={index}
								className="
									w-full h-min-[150px] shadow-lg 
									p-3
									bg-white mt-1 rounded-lg
									border-2
								"
							>
								<div>
									<strong>Name</strong>
								</div>
								<input
									type="text"
									value={data?.label || ''}
									placeholder="Label"
									onChange={(e) =>
										updateNodeData(id, { label: e.target.value })
									}
									className="border p-1 w-full mt-2 rounded-lg"
								/>
								{data.config ?
								(
									<div>
										<div className="mt-1">
												<div>
													<strong>Command</strong>
												</div>
												<input
													type="text"
													placeholder="Command"
													value={data.config.command || ''}
													className="border p-1 w-full mt-2 rounded-lg"
												/>
										</div>
										<div className="mt-1">
												<div>
													<strong>Environment</strong>
												</div>
												<input
													type="text"
													placeholder="Environment"
													value={data.config.environment || ''}
													className="border p-1 w-full mt-2 rounded-lg"
												/>
										</div>
										<div className="mt-1">
												<div>
													<strong>Image</strong>
												</div>
												<input
													type="text"
													placeholder="Image"
													value={data.config.image || ''}
													className="border p-1 w-full mt-2 rounded-lg"
												/>
										</div>
										<div className="mt-1">
												<div>
													<strong>Ports</strong>
												</div>
												<input
													type="text"
													placeholder="Ports"
													value={data.config.ports || ''}
													className="border p-1 w-full mt-2 rounded-lg"
												/>
										</div>
										<div className="mt-1">
												<div>
													<strong>Volumns</strong>
												</div>
												<input
													type="text"
													placeholder="Volumns"
													value={data.config.volumes || ''}
													className="border p-1 w-full mt-2 rounded-lg"
												/>
										</div>
									</div>
								) 
								: ""}
							</div>
						);
					})}
				</div>
			</div>
		</div>
	);
};

export default LeftBar;