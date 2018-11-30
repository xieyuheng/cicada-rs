<template>
    <div id="app">
        <NoteBook :note_list="init_note_list"></NoteBook>
    </div>
</template>

<script>
 import Vue from "vue";
 import NoteBook from "./NoteBook.vue";

 export default {
     components: {
         NoteBook,
     },
     data: function () {
         return {
             init_note_list: [
                 { headline: "#1", input: "asd", output: "sad" },
                 { headline: "#2", input: "asd", output: "sad" },
             ]
         }
     },
 }
</script>

<style>
 #app {
     font-family:
         "Avenir",
         "Helvetica, Arial",
         "sans-serif";
     -webkit-font-smoothing: antialiased;
     -moz-osx-font-smoothing: grayscale;
     /* text-align: center; */
 }
</style>
