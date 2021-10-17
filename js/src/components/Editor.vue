<template>
    <codemirror :options="cmSettings" v-model="code" ref="cm"></codemirror>
</template>

<script>

import { codemirror } from 'vue-codemirror';
import 'codemirror/lib/codemirror.css';
import 'codemirror/mode/brainfuck/brainfuck';
import 'codemirror-one-dark-theme/one-dark.css'

export default {
  name: 'Editor',
  data() {
    return {
        cmSettings: {
            mode: 'text/x-brainfuck',
            theme: 'one-dark',
            lineNumbers: true,
            matchBrackets: true,
            styleSelectedText: true,
            styleActiveLine: true
        },
        code: ''
    }
  },
  components: {
    codemirror
  },
  methods: {
    getCode() {
        return this.code;
    }
  },
  mounted() {
    this.code = localStorage.code ? localStorage.code : "";
  },
  computed: {
    cm() {
        return this.$refs.cm;
    }
  }
}
</script>

<style>

</style>
