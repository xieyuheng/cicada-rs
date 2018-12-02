<template>
    <div class="Note">
        <h3>{{ headline }}
            <button class="button-run"
                    @click="$emit ('run', state.index)">
                RUN
            </button>
            <button class="button-new"
                    @click="$emit ('new', state.index)">
                NEW
            </button>
        </h3>
        <textarea
            v-model="state.input"
            ref="input"
            class="input"
            @input="$emit ('input_change', state.input)"
            @keydown.ctrl.enter="$emit ('run', state.index)"
            @keydown.alt.enter="$emit ('new', state.index)"
            @focus="$emit ('focus', state.index)"
        />
        <pre class="output" v-if="output">{{ output }}</pre>
        <hr />
    </div>
</template>

<script>
 import Vue from "vue";

 export default {
     props: [
         "index",
         "headline",
         "input",
         "output",
     ],
     data: function () {
         return {
             state: {
                 input: this.input,
                 index: this.index,
             }
         }
     },
     methods: {
         focus () {
             this.$refs.input.focus ();
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

 .input {
     font-size: 1em;
     border: 0px;
     border-left: 4px solid #aaa;
     width: 97vw;
     height: 40vh;
 }

 .output {
     font-size: 1em;
     border: 0px;
     border-left: 4px solid #8cf;
     width: 97vw;
 }

 .button-run {
     font-size: 1em;
     border: 0px;
     border-right: 3px solid #8cf;

 }

 .button-new {
     font-size: 1em;
     border: 0px;
     border-right: 3px solid #aaa;
 }
</style>
