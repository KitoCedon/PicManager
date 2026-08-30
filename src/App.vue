<script setup lang="ts">
import { ref } from "vue";
import UploadView from "./views/UploadView.vue";
import PreviewView from "./views/PreviewView.vue";
import ConfigView from "./views/ConfigView.vue";

const current = ref("preview");

const nav = [
	{ key: "upload", title: "上传", icon: "mdi-upload" },
	{ key: "preview", title: "预览", icon: "mdi-image-multiple" },
	{ key: "config", title: "设置", icon: "mdi-cogs" },
];
</script>

<template>
	<v-app>
		<v-navigation-drawer permanent :width="220">
			<v-list nav density="comfortable" v-model:selected="current">
				<v-list-item
					v-for="item in nav"
					:key="item.key"
					:value="item.key"
					:title="item.title"
					:prepend-icon="item.icon"
				/>
			</v-list>
		</v-navigation-drawer>

		<v-main>
			<v-container fluid>
				<UploadView v-if="current == 'upload'" />
				<PreviewView v-else-if="current == 'preview'" />
				<ConfigView v-else-if="current == 'config'" />
			</v-container>
		</v-main>
	</v-app>
</template>
