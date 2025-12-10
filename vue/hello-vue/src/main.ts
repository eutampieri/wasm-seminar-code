import { createApp } from 'vue'
import App from './App.vue'
import loadWasm from 'hello_world';

loadWasm().then(() => createApp(App).mount('#app'));
