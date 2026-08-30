<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Bed {
	id: string;
	name: string;
	type: string;
	host: string;
	port: number;
	username: string;
	password: string;
	remote_dir: string;
	image_url: string;
	local_dir: string;
}
const bedList = ref<Bed[]>([]);
onMounted(async () => {
	try {
		bedList.value = await invoke<Bed[]>("get_bed_list");
	} catch (e) {
		console.error("get_bed_list failed:", e);
	}
});
</script>

<template>
	<div>
		<v-list>
			<v-list-item
				v-for="bedItem in bedList"
				:key="bedItem.id"
				:value="bedItem.id"
				:title="bedItem.name"
			></v-list-item>
		</v-list>
	</div>
</template>
