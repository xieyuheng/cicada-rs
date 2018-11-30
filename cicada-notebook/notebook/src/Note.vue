<template>
    <div class="Note">
        <hr />
        <h3>{{ headline }}</h3>
        <textarea
            class="InputBuffer"
            v-bind:value="input"
            v-on:keydown="onkeydown">
        </textarea>
        <textarea
            class="OutputBuffer"
            v-if="output"
            v-bind:value="output">
        </textarea>
        <hr />
    </div>
</template>

<script>
 import Vue from "vue";

 export default {
     props: [
         "headline",
         "focus_p",
         "input",
         "output",
     ],
     methods: {
         onkeydown (event) {
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
     border-left: 4px solid #aae;
     width: 97vw;
     height: 40vh;
 }
</style>
