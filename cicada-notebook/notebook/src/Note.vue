<template>
    <div class="Note">
        <h3>{{ headline }}</h3>
        <button @click="focus">focus</button>
        <textarea
            v-model="state.input"
            ref="input_buffer"
            class="InputBuffer"
            @input="$emit ('input_change', state.input)"
            @keydown.ctrl.enter="$emit ('run')"
            @keydown.alt.enter="$emit ('next')"
        />
        <textarea
            class="OutputBuffer"
            v-if="output"
            :value="output"
        />
        <hr />
    </div>
</template>

<script>
 import Vue from "vue";

 export default {
     props: [
         "headline",
         "input",
         "output",
     ],
     data: function () {
         return {
             state: {
                 input: this.input,
             }
         }
     },
     methods: {
         focus () {
             this.$refs.input_buffer.focus ();
         }
     },
 }
</script>

<style scoped>
 textarea {
     font-family:
         "Monaco",
         "Input",
         "monofur",
         "Hack",
         "Noto Mono",
         "Consolas",
         "Lucida Console", "Andale Mono WT",
         "Andale Mono", "Lucida Console",
         "Lucida Sans Typewriter",
         "DejaVu Sans Mono",
         "Bitstream Vera Sans Mono",
         "Liberation Mono",
         "Nimbus Mono L",
         "Courier New",
         "Courier",
         "monospace";
 }

 h3 {
     color: #888;
 }

 .InputBuffer {
     font-size: 1em;
     border: 0px;
     border-left: 4px solid #aaa;
     width: 97vw;
     height: 40vh;
 }

 .OutputBuffer {
     font-size: 1em;
     border: 0px;
     border-left: 4px solid #8cf;
     width: 97vw;
     height: 40vh;
 }
</style>
