<template>
  <h1>Callback</h1>
</template>

<script setup lang="ts">
// import * as config from "@/config";
import { onMounted } from 'vue'
let serverUrl = `http://localhost:5000/api`

onMounted(() => {
  console.log("callback页面已经加载完毕");
  let code = getCode();
  console.log("code: " + code);
  getUserInfo(code);
})
// get url params
const getCode = () => {
  console.log("从回调地址中获取code");
  let url = window.location.href
  console.log("回调url: " + url);
  let params = url.split('?')[1]
  // 获取code
  for (let element of params.split('&')) {
    if (element.split('=')[0] == "code") {
      return element.split('=')[1]
    }
  }
  return "";
}
const getUserInfo = (code: any) => {
  fetch(`${serverUrl}/auth/${code}`, {
    method: 'GET',
  }).then(res => {
    res.json().then(data => {
      console.log("获取用户信息成功", data);
      localStorage.setItem('user', JSON.stringify(data.user));
      localStorage.setItem('token', data.token);
      window.location.href = "/home";
    });
  });
}
</script>

<style scoped lang="scss"></style>
