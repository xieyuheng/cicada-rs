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
//   ctrl + enter -- RUN
//   alt  + enter -- NEW
`;

 export default {
     components: {
         NoteBook,
     },
     data: function () {
         return {
             init_note_list: [
                 { id: 0, input: WELCOME_MESSAGE },
             ]
         }
     },
 }
</script>

<style>
 #app {
     -webkit-font-smoothing: antialiased;
     -moz-osx-font-smoothing: grayscale;
 }
</style>
