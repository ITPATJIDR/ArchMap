import React, { useEffect, useState } from 'react';
import { IconType } from 'react-icons';
import { FaCircleNodes } from "react-icons/fa6";
import { FaDocker } from "react-icons/fa";
import { invoke } from '@tauri-apps/api/tauri';

type IconItem = {
	name: string;
	icons: string;
};

type TabItem = {
	name: string;
	icons_component: IconType;
};

type IconList = IconItem[];

type TabList = TabItem[];

interface RightNavbarProps {
  addNode: (image: string, name: string, service: string) => void;
}

const RightNavbar: React.FC<RightNavbarProps> = ({ addNode }) => {

	const [selectTab, setSelectTab] = useState<string>("services");
	const [repositories, setRepositories] = useState<string[]>([]);
	const [loading, setLoading] = useState(false);
	const [searchTerm, setSearchTerm] = useState<string>("");
	console.log(loading)

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
			icons: "https://avatars.githubusercontent.com/u/15729372?s=280&v=4"
		},
		{
			name: "Elysia",
			icons: "https://elysiajs.com/assets/elysia.svg"
		},
		{
			name: "Vue",
			icons: "https://static-00.iconduck.com/assets.00/vue-icon-2048x1766-ntogpmti.png"
		},
		{
			name: "Angular",
			icons: "https://upload.wikimedia.org/wikipedia/commons/thumb/c/cf/Angular_full_color_logo.svg/2048px-Angular_full_color_logo.svg.png"
		},
		{
			name: "Svelte",
			icons: "https://upload.wikimedia.org/wikipedia/commons/thumb/c/cf/Angular_full_color_logo.svg/2048px-Angular_full_color_logo.svg.png"
		},
		{
			name: "Next",
			icons: "https://static-00.iconduck.com/assets.00/nextjs-icon-1024x1024-5et230l7.png"
		},
		{
			name: "Nuxt",
			icons: "https://nuxt.com/assets/design-kit/icon-green.svg"
		},
		{
			name: "Django",
			icons: "https://www.svgrepo.com/show/353657/django-icon.svg"
		},
		{
			name: "FastAPI",
			icons: "https://cdn.worldvectorlogo.com/logos/fastapi.svg"
		},
	];

	const tab_list: TabList = [
		{
			name: "services",
			icons_component: FaCircleNodes
		},
		{
			name: "docker-compose",
			icons_component: FaDocker
		},
	];

	const filteredIconList = icon_list.filter(icon =>
		icon.name.toLowerCase().includes(searchTerm.toLowerCase())
	);

	const filteredDockerRepoList = repositories.filter(repo =>
		repo.toLowerCase().includes(searchTerm.toLowerCase())
	);

	useEffect(() => {
		let isMounted = true;

		const fetchDockerRepositories = async () => {
			if (filteredDockerRepoList.length === 0) {
				try {
					const result = await invoke<string[]>('search_docker_repositories', {
						query: searchTerm,
						page: 1,
						pageSize: 10
					});
					if (isMounted) {
						setRepositories(result);
					}
				} catch (error) {
					if (isMounted) {
						console.error("Error fetching Docker repositories:", error);
					}
				}
			}
		};

		fetchDockerRepositories();

		return () => {
			isMounted = false;
		};
	}, [filteredDockerRepoList]);


	const tab_data = (tab: string) => {
		if (tab === "services") {
			return (
				<div className='h-[83vh] overflow-auto'>
					<div className='flex flex-wrap'>
						{filteredIconList.map(({ name, icons }: IconItem, index) => {
							return (
								<button
									key={index}
									onClick={() => addNode(icons, name, selectTab)} // Pass the image URL
									className="w-[65px] h-[70px] p-2 m-2 bg-gray-100 rounded-lg hover:bg-gray-200 flex items-center justify-center"
								>
									<div className='flex flex-col items-center'>
										<img className="w-8 h-8" src={icons} alt="icon" />
										<div className='mt-1'>
											{name}
										</div>
									</div>
								</button>
							);
						})}
					</div>
				</div>
			);
		} else if (tab === "docker-compose") {
			return (
				<div className='h-[83vh] overflow-auto'>
					<div className='flex flex-wrap'>
						{filteredDockerRepoList.map((item: string, index) => {
							return (
								<button
									key={index}
									onClick={() => addNode("https://cdn-icons-png.flaticon.com/512/919/919853.png", item, selectTab)} // Pass the image URL
									className="w-[65px] h-[65px] p-2 m-2 bg-gray-100 rounded-lg hover:bg-gray-200 flex items-center justify-center"
								>
									<div className='flex flex-col items-center'>
										<img className="w-8 h-8" src={"https://cdn-icons-png.flaticon.com/512/919/919853.png"} alt="icon" />
										<div className='mt-1 truncate w-16 h-5'>
											{item}
										</div>
									</div>
								</button>
							);
						})}
					</div>

				</div>
			);
		}
	};

	const fetchRepositories = async () => {
		setLoading(true);
		try {
			const result = await invoke<string[]>('fetch_docker_repositories', {
				page: 1,
				pageSize: 30,
			});
			setRepositories(result);
		} catch (error) {
			console.error('Error fetching repositories:', error);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchRepositories();
	}, [searchTerm]);

	return (
		<div className="w-[250px] h-[90vh] absolute top-20 left-4 space-y-2 z-10 flex bg-white shadow-lg rounded-lg">
			<div className='w-[50px] p-2 border-r-2'>
				{tab_list.map((item: TabItem) => {
					return (
						<div
							key={item.name}
							className='
							mb-3 mt-3 w-[33px] h-[33px] flex 
							items-center justify-center'
							onClick={() => setSelectTab(item.name)}
						>
							<item.icons_component />
						</div>
					);
				})}
			</div>
			<div>
				<div className='p-2'>
					<div className='w-full'>
						<input
							className='w-full border-2 p-1 rounded-lg'
							type="text"
							placeholder='Search'
							value={searchTerm}
							onChange={(e) => setSearchTerm(e.target.value)}
						/>
					</div>
					<div className='pl-3 mt-3'>
						{tab_data(selectTab)}
					</div>
				</div>
			</div>
		</div>
	);
};

export default RightNavbar;
