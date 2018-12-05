<template>
    <div id="app">
        <NoteBook/>
    </div>
</template>

<script>
 import Vue from "vue";
 import NoteBook from "./NoteBook.vue";

 export default {
     components: {
         NoteBook,
     },
 }
</script>

<style>
 #app {
     -webkit-font-smoothing: antialiased;
     -moz-osx-font-smoothing: grayscale;
 }
</style>
