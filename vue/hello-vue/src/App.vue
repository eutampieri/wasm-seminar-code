<script setup lang="ts">
import { computed, ref } from 'vue';

import { hello, add } from 'hello_world';

const name = ref("");
const title = computed(() => hello(name.value));

const left = ref(0);
const right = ref(0);
const sum = computed(() => add(BigInt(left.value), BigInt(right.value)))
</script>

<template>
  <h1>{{ title }}</h1>
  <section>
    <label for="name">Your name:</label>
    <input type="text" id="name"></input>
  </section>
  <h2>Math</h2>
  <section>
    <input type="number" v-model="left" /> + <input type="number" v-model="right" /> = {{ sum }}
  </section>
</template>

<style scoped></style>
