<template>
    <div class="NoteBook">
        <Note
            v-for="(note, index) in state.note_list"
            ref="note_list"
            :key="index"
            :headline="default_headline (index)"
            :input="note.input"
            :output="note.output"
            @input_change="change_input (index, $event)"
            @run="run (index)"
            @next="next (index)"
        />
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
     data: function () {
         return {
             state: {
                 note_list: this.note_list,
             }
         }
     },
     methods: {
         default_headline (index) {
             return "#" + (index + 1).toString ();
         },
         change_input (index, input) {
             let note = this.state.note_list [index];
             note.input = input;
         },
         run (index) {
             cicada.then (cicada => {
                 let module = cicada.CicadaModule.new ();
                 let list = this.state.note_list;
                 let length = list.length;
                 list.forEach ((note, i) => {
                     if (i <= index) {
                         note.output =
                             module.run (note.input);
                     }
                 });
                 this.state.note_list = list.slice (0, length);
             })
         },
         next (index) {
             this.run (index);
             let list = this.state.note_list;
             let length = list.length;
             let ante = list.slice (0, index + 1);
             let succ = list.slice (index + 1, length);
             ante.push ({});
             this.state.note_list = ante.concat (succ);
             let note = this.$refs.note_list [index + 1];
             note.focus ();
         },
     }
 }
</script>

<style scoped>

</style>
