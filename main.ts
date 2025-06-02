import Casdoor from 'casdoor-vue-sdk';
import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import App from './App.vue'
import './registerServiceWorker'
import { createPinia } from 'pinia'
import router from './router'

// const config = {
//     serverUrl: "http://localhost:5000/api",
// };
// const config = {
//     serverUrl: "https://door.casdoor.com",
//     clientId: "294b09fbc17f95daf2fe",
//     organizationName: "casbin",
//     appName: "app-vue-python-example",
//     redirectPath: "/callback",
// };
// const config = {
//     serverUrl: "http://localhost:7001/",
//     clientId: "c5cbfaf6792eb12163a4",
//     organizationName: "casbin",
//     appName: "neat_work_space",
//     redirectPath: "/callback",
// };
const config = {
    serverUrl: "http://localhost:7001/",
    // serverUrl: "https://door.casdoor.com",
    clientId: "294b09fbc17f95daf2fe",
    organizationName: "casbin",
    appName: "neat_work_space",
    redirectPath: "/callback",
};
const app = createApp(App)


app.use(Casdoor, config);
const pinia = createPinia();
for (const [key, component] of Object.entries(ElementPlusIconsVue)) { app.component(key, component) }

app.use(ElementPlus, { locale: zhCn }).use(router).use(pinia).mount('#app')
