<template>
  <div class="container">
    <h4>Parse result</h4>
    <pre>{{ parseResult }}</pre>

    <br />
    <textarea v-model="expression" rows="10" cols="60" />
  </div>
</template>

<script setup>
import { computed, ref } from 'vue';
import { Parser } from '../pkg';

const expression = ref('a == 1 && b == 2')
const parseResult = computed(() => Parser.parse(expression.value))
</script>

<style>
.container {
  max-width: 600px;
  margin: 20px;
  text-align: left;
}

textarea {
  font-family: monospace;
}

pre {
  text-align: left;
}
</style>
