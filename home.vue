<template>
  <div>
    <div>
      <h1>我是简介办公的home</h1>
      <table border="1">
        <tr>
          <th>Name</th>
          <th>Value</th>
        </tr>
        <tr>
          <td>user name</td>
          <td>{{ account.name }}</td>
        </tr>
        <tr>
          <td>user avatar</td>
          <td><img :src="account.avatar" alt="user avatar" style="width:50px" /></td>
        </tr>
        <tr>
          <td>user id</td>
          <td>{{ account.id }}</td>
        </tr>
        <tr>
          <td>user owner</td>
          <td>{{ account.owner }}</td>
        </tr>
        <tr>
          <td>user created time</td>
          <td>{{ account.createdTime }}</td>
        </tr>
      </table>
    </div>
    <br>
    <button @click="logout">Sign out</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
const account = ref({ name: '', avatar: '', id: '', owner: '', createdTime: '' });
onMounted(() => {
  let user = localStorage.getItem('user');
  console.log("data: " + user);
  account.value = JSON.parse(user);
  console.log("account: " + account.value.name);
});
const logout = () => {
  localStorage.removeItem('user');
  window.location.href = "/";
}
</script>

<style scoped lang="scss"></style>
