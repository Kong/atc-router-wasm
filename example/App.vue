<template>
  <div class="container">
    <h4>Parse result</h4>
    <pre>{{ parseResult }}</pre>

    <br />
    <textarea v-model="expression" rows="10" cols="60" />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { AstType, Parser, Schema } from '../pkg'

const createSchema = (fieldMapping: Record<string, string[]>) => {
  const schema = new Schema()
  for (const [type, fields] of Object.entries(fieldMapping)) {
    for (const field of fields) {
      schema.addField(field, type as AstType)
    }
  }
  return schema
}

const HTTP_SCHEMA = createSchema({
  String: [
    'net.protocol',
    'tls.sni',
    'http.method',
    'http.host',
    'http.path',
    'http.path.segments.*',
    'http.headers.*',
    'http.queries.*',
  ],
  Int: ['net.src.port', 'net.dst.port', 'http.path.segments.len'],
  IpAddr: ['net.src.ip', 'net.dst.ip'],
})

const expression = ref('http.host == "localhost" && !(net.src.port == 8080)')
const parseResult = computed(() => Parser.parse(expression.value, HTTP_SCHEMA))
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
