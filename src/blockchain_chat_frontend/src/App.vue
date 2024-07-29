<script lang="ts">
import { HttpAgent } from '@dfinity/agent';
import { blockchain_chat_backend } from '../../declarations/blockchain_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import type { Identity } from '@dfinity/agent';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[],
      identity: undefined as undefined | Identity,
    }
  },
  methods: {
    async addNote() {
      await blockchain_chat_backend.add_note(this.newNote);
      await this.downloadNotes();
    },
    async downloadNotes() {
      this.notes = await blockchain_chat_backend.get_notes();
    },
    async logIn(){
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      });

      const identity = authClient.getIdentity();
      console.log(`User has logged in: ${identity.getPrincipal()}`);
      this.identity = identity;
      // user personal id -> You can take id or use this identity to authorize
      // const agent = new HttpAgent({identity} );
    }
  },
  mounted() {
    this.downloadNotes();
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div>
      {{ identity?.getPrincipal() }}
      <button @click="logIn()">Log In</button>
    </div>
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
