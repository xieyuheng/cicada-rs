<template>
    <div class="NoteBook">
        <Note
            v-for="note in note_list"
            :headline="note.headline"
            :input="note.input"
            :output="note.output" />
    </div>
</template>

<script>
 import Vue from "vue";
 import Note from "./Note.vue";
 const cicada = import ("../wasm_modules/cicada_notebook");

 export default {
     components: {
         Note,
     },
     props: [ "note_list" ],
     methods: {
         onkeydown (event) {
             cicada.then (cicada => {
                 console.log (cicada);
             })
         }
     }
 }
</script>

<style scoped>

</style>
