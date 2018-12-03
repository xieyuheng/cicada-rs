<template>
    <div>
        <p>
            <input class="headline"
                   type="text"
                   size="16"
                   v-model="state.headline"
                   @input="$emit ('headline_change', state.headline)"
                   @keydown.enter="$emit ('headline_enter')"
            />
            <button class="button"
                    @click="$emit ('run', state.index)">
                RUN
            </button>
            <button class="button"
                    @click="$emit ('new', state.index)">
                NEW
            </button>
        </p>
        <textarea
            class="input"
            spellcheck="false"
            wrap="off"
            rows="18"
            ref="input"
            v-model="state.input"
            @input="$emit ('input_change', state.input)"
            @keydown.ctrl.enter="$emit ('run', state.index)"
            @keydown.alt.enter="$emit ('new', state.index)"
            @focus="$emit ('focus', state.index)"
        />
        <pre
            class="output"
            v-if="output">{{ output }}</pre>
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
     data () {
         return {
             state: {
                 headline: this.headline,
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
 .input {
     font-size: 1em;
     border: 0px;
     border-left: 3px solid;
     width: 97%;
 }

 .output {
     font-size: 1em;
     border: 0px;
     border-left: 3px solid;
     overflow-x: auto;
 }

 .button {
     float: right;
     font-size: 1em;
     border: 0px;
     border-right: 2px solid;
     background-color: white;
 }

 .headline {
     font-size: 1em;
     border: 0px;
 }
</style>
