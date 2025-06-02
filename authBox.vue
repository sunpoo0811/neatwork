<template>
  <div>
    <button @click="signup">Sign up</button>
    <button @click="login">Sign in</button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
// router.push("/home");

let serverUrl = `http://localhost:5000/api`

const login = () => {
  fetch(`${serverUrl}/login`, {
    method: 'GET',
  }).then(res => {
    res.json().then(data => {
      console.log(data)
      window.location.href = data
    });
  });
};

const signup = () => {
  fetch(`${serverUrl}/signup`, {
    method: 'GET',
  }).then(res => {
    res.json().then(data => {
      console.log(data)
      window.location.href = data
    });
  });
};
</script>

<style scoped lang="scss"></style>
