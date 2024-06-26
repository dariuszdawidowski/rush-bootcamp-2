<script setup>
import { ref } from 'vue';
import { rush_bootcamp_2_backend } from 'declarations/rush-bootcamp-2-backend/index';
import Blog from './components/Blog.vue';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const last_name = target.querySelector('#last_name').value;
  await rush_bootcamp_2_backend.greet(name, last_name).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <h1 class="text-blue-600">Hello</h1>
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <input id="last_name" alt="Last Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
    <Blog />
  </main>
</template>
