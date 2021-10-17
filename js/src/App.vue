<template>
    <main class="flex flex-col h-screen">
        <div class="flex items-center text-xl bg-darkgray">
            <button class="text-5xl text-green bi bi-play-fill" v-on:click="run"></button>
        </div>
        <editor ref="editor"></editor>
        <div class="p-2 bg-darkgray text-lightgray">
        Console
        </div>
        <console></console>
    </main>
</template>

<script>
import editor from "./components/Editor.vue";
import console from "./components/Console.vue";
import { Kernel } from "./kernel/Kernel.js"; 
import "bootstrap-icons/font/bootstrap-icons.css"

export default {
  name: 'App',
  data() {
    return {
        output: [],
    }
  },
  computed : {
    cm() {
        return this.$refs.editor.cm.codemirror;
    },
    code() {
        return this.$refs.editor.getCode();
    }
  },
  components: {
    editor,
    console
  },
  methods: {
    run() {
        var kernel = new Kernel(this.code);
        this.output = kernel.parse();
        localStorage.code = this.code;
        var last_output;
        if ((last_output = this.output[this.output.length - 1]))
            if (last_output.type == "error")
                this.cm.markText({line: kernel.last_line, ch: kernel.last_token-1}, {line: kernel.last_line, ch: kernel.last_token}, {css: "color: #e06c75 !important"});
    }
  }
}
</script>

<style>

</style>
