<template>
    <div :class="{ toc: state.toc_p }">
        <NavBar :heads="heads" />
        <div class="content">
            <MenuBar
                @toc="toggle_toc"
            />
            <Note
                v-for="(note, index) in state.note_list"
                :id="id_of_note (note, index)"
                ref="note_list"
                :key="note.id"
                :headline="headline_of_note (note, index)"
                :index="index"
                :input="note.input"
                :output="note.output"
                @input_change="change_input (index, $event)"
                @headline_change="change_headline (index, $event)"
                @headline_enter="enter_headline (index)"
                @run="run_note ($event)"
                @new="new_note ($event)"
                @focus="set_focus ($event)"
            />
        </div>
    </div>
</template>

<script>
 import Vue from "vue";
 import Note from "./Note.vue";
 import NavBar from "./NavBar.vue";
 import MenuBar from "./MenuBar.vue";
 const cicada = import ("../wasm_modules/cicada_notebook");

 export default {
     components: {
         Note,
         NavBar,
         MenuBar,
     },
     props: [ "note_list" ],
     // note_t: {
     //     id: "number",
     //     headline: "string",
     //     input: "string",
     //     output: "string",
     // },
     data () {
         return {
             state: {
                 note_list: this.note_list,
                 focus: 0,
                 counter: this.note_list.length,
                 toc_p: false,
                 nop: "",
             }
         }
     },
     computed: {
         heads () {
             return this.state.note_list.map ((note, index) => {
                 let id = "#" + index.toString ();
                 if (note.headline) {
                     return { line: note.headline,
                              id: id };
                 } else {
                     return { line: id,
                              id: id };
                 }
             });
         },
     },
     methods: {
         id_of_note (note, index) {
             return index.toString ();
             // if (note.headline) {
             //     // need to change line to lisp-case
             //     return note.headline;
             // } else {
             //     return index.toString ();
             // }
         },
         headline_of_note (note, index) {
             if (note.headline) {
                 return note.headline;
             } else {
                 return "#" + (index.toString ());
             }
         },
         change_input (index, input) {
             let note = this.state.note_list [index];
             note.input = input;
         },
         change_headline (index, headline) {
             // we need should not update page here
             //   because after updated
             ///  the focus will be set to state.focus
             this.set_focus (index);
             let note = this.state.note_list [index];
             note.headline = headline;
         },
         enter_headline (index) {
             // note that, after updated
             ///  the focus will be set to state.focus
             this.set_focus (index);
             let note = this.state.note_list [index];
             Vue.set (this.state.note_list, index, note);
             // can not even use `this.$forceUpdate ()` here
             //   because only effect on this.state.note_list
             //   will fire `this.heads ()`
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
         },
         toggle_toc () {
             if (this.state.toc_p) {
                 this.state.toc_p = false;
             } else {
                 this.state.toc_p = true;
             }
         },
     },
     updated () {
         // we need to set focus in after updated
         //   because before that
         //   the dom we want to focus on is not created yet
         // we can not just index the array of components
         //   because it is not ordered by index
         for (let note of this.$refs.note_list) {
             if (note.index == this.state.focus) {
                 note.focus ();
             }
         }
     },
 }
</script>

<style scoped>
 .content {
     margin-left: 0%;
 }

 .NavBar {
     width: 0%;
 }

 .toc .content {
     margin-left: 18%;
 }

 .toc .NavBar {
     width: 18%;
 }
</style>
