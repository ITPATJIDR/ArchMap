// src/components/RightNavbar.tsx
import React from 'react';

type IconItem = {
	name: string
	icons: string;
};

type IconList = IconItem[];

interface RightNavbarProps {
  addNode: (image: string, name:string) => void;
}

const RightNavbar: React.FC<RightNavbarProps> = ({ addNode }) => {
	const icon_list: IconList = [
		{
			name: "React",
			icons: "https://cdn4.iconfinder.com/data/icons/logos-3/600/React.js_logo-512.png"
		},
		{
			name: "Express",
			icons: "https://img.icons8.com/color/512/express-js.png"
		},
		{
			name: "Gin",
			icons: "https://seeklogo.com/images/G/gin-logo-BD71D14076-seeklogo.com.png"
		},
	];

	return (
		<div className="w-[200px] h-[650px] absolute top-20 left-4 space-y-2 z-10 flex flex-col bg-white shadow-lg p-2 rounded-lg">
			<div className='p-5 flex flex-wrap'>
				{icon_list.map(({ name, icons }: IconItem, index: number) => {
					return (
						<button
							key={index}
							onClick={() => addNode(icons, name)} // Pass the image URL
							className="w-[55px] h-[55px] p-2 m-2 bg-gray-100 rounded-lg hover:bg-gray-200 flex items-center justify-center"
						>
							<img className="w-8 h-8" src={icons} alt="icon" />
						</button>
					);
				})}
			</div>
		</div>
	);
};

export default RightNavbar;
