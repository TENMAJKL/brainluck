<template>
   <div class="h-full p-2 overflow-y-auto break-words text-lightgray bg-midgray">
            <span v-for="(output, index) in this.outputs" v-bind:key="index">
                <br v-if="output.content == '\n'">
                <div v-else-if="output.type == 'error'" class="p-2 border-2 border-red">
                    {{ output["content"] }}
                </div>
                <span v-else>{{ output["content"]}}</span>
            </span>
   </div>
</template>

<script>
export default {
    computed: {
        outputs() {
            return this.$parent.output;
        },
        kernel() {
            return this.$parent.kernel;
        }
    }

}
</script>

