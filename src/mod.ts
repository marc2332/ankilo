import { loadPlugin, importFromPlugin } from 'https://denopkg.com/Srinivasa314/calcite-ts@1.0/calcite.ts';
await loadPlugin("ankilo", "file://target/debug")
const createApp = importFromPlugin('createApp')

class App {
	private window_name: String;
	constructor(window_name: string){
		this.window_name = window_name
	}
	public launch(){
		createApp(this.window_name)
	}
}

export {
	App
}