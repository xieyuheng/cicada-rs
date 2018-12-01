<template>
    <div class="NoteBook">
        <Note
            v-for="(note, index) in state.note_list"
            ref="note_list"
            :key="note.id"
            :headline="default_headline (index)"
            :index="index"
            :input="note.input"
            :output="note.output"
            @input_change="change_input (index, $event)"
            @run="run_note ($event)"
            @new="new_note ($event)"
            @focus="set_focus ($event)"
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
     // note_t: {
     //     id: "number",
     //     headline: "string",
     //     input: "string",
     //     output: "string",
     // },
     data: function () {
         return {
             state: {
                 note_list: this.note_list,
                 focus: 0,
                 counter: 1,
             }
         }
     },
     methods: {
         default_headline (index) {
             return "#" + (index.toString ());
         },
         change_input (index, input) {
             let note = this.state.note_list [index];
             note.input = input;
         },
         run_note (index) {
             cicada.then (cicada => {
                 let module = cicada.CicadaModule.new ();
                 let list = this.state.note_list;
                 list.forEach ((note, i) => {
                     if (i <= index) {
                         note.output = module.run (note.input);
                     }
                 });
                 this.state.note_list = list.slice (0, list.length);
             })
         },
         new_id () {
             this.state.counter = this.state.counter + 1;
             return this.state.counter;
         },
         new_note (index) {
             this.run_note (index);
             let list = this.state.note_list;
             let ante = list.slice (0, index + 1);
             let succ = list.slice (index + 1, list.length);
             ante.push ({ id: this.new_id () });
             this.state.note_list = ante.concat (succ);
             this.set_focus (index + 1);
         },
         set_focus (index) {
             this.state.focus = index;
         }
     },
     updated: function () {
         for (let note of this.$refs.note_list) {
             if (note.index == this.state.focus) {
                 note.focus ();
             }
         }
     },
 }
</script>

<style scoped>

</style>
