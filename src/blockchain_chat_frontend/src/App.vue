<script lang="ts">
import { ref } from 'vue';
import { blockchain_chat_backend } from '../../declarations/blockchain_chat_backend';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[],
    }
  },
  methods: {
    async addNote() {
      await blockchain_chat_backend.add_note(this.newNote);
      await this.downloadNotes();
    },
    async downloadNotes() {
      this.notes = await blockchain_chat_backend.get_notes()
    }
  },
  mounted() {
    this.downloadNotes()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div>
      {{ notes }}
    </div>
    <div>
      <textarea v-model="newNote" placeholder="Add new note..."></textarea>
      <button @click="addNote()">Add new note</button>
    </div>
    <div>
      {{ newNote }}
    </div>
  </main>
</template>
