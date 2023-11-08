import init, {
	api1,
	custom_main,
	new_store,
} from './libs/lib_wasm.js';

(async () => {
	await init();
	console.log(api1());
	custom_main();
	let store = new_store();
	console.log(store);
	store.add("a", 1);
	store.add("b", 3);
	store.add("c", 5);
	console.log(store.get("b"));
})();
