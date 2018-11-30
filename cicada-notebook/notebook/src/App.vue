<template>
    <div id="app">
        <NoteBook :note_list="init_note_list"></NoteBook>
    </div>
</template>

<script>
 import Vue from "vue";
 import NoteBook from "./NoteBook.vue";

 const WELCOME_MESSAGE = `\
// welcome to cicada-notebook ^-^/
// - short keys :
//   ctrl + enter -- run
//   alt  + enter -- next
`;

 export default {
     components: {
         NoteBook,
     },
     data: function () {
         return {
             init_note_list: [
                 { input: WELCOME_MESSAGE },
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
 }
</style>
