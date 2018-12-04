<template>
    <div class="MenuBar">
        <button class="button"
                @click="$emit ('toc')">
            #
        </button>
        <button class="button"
                @click="click_load">
            LOAD
        </button>
        <input ref="load"
               type=file
               accept=".cic,.txt"
               style="display: none"
               @change="load_file ($event)" />
        <button class="button"
                @click="$emit ('save')">
            SAVE
        </button>
    </div>
</template>

<script>
 import Vue from "vue";

 export default {
     methods: {
         click_load () {
             this.$refs.load.click ();
         },
         load_file (event) {
             let file = event.target.files [0];
             let reader = new FileReader ();
             console.log (`- loading file`);
             console.log (`  name = ${file.name}`);
             console.log (`  size = ${file.size}`);
             reader.readAsText (file);
             reader.onload = (e) => {
                 this.$emit ("load", e.target.result);
             };
         },
     },
 }
</script>

<style scoped>
 .MenuBar {
     position: sticky;
     top: 0;
 }

 .button {
     font-size: 1em;
     border: 0px;
     border-right: 2px solid;
     background-color: white;
 }
</style>
