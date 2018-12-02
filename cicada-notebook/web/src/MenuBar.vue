<template>
    <div class="MenuBar">
        <button class="button"
                @click="$emit ('toc')">
            @
        </button>
        <button class="button"
                @click="$emit ('save')">
            SAVE
        </button>
        <button class="button"
                @click="$emit ('load')">
            LOAD
        </button>
    </div>
</template>

<script>
 import Vue from "vue";

 export default {
     props: [
     ],
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
