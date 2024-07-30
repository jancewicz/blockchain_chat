<script lang="ts">
import { HttpAgent, verify } from '@dfinity/agent';
import { blockchain_chat_backend, canisterId, createActor } from '../../declarations/blockchain_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[][],
      identity: undefined as undefined | Identity,
      principalName: "",
    }
  },
  methods: {
    verifyUser() {
      if(!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("User is not logged in");
      }
    },
    async addNote() {
      this.verifyUser();
      const backend = createActor(canisterId, {agentOptions: {identity: this.identity}});
      await backend.add_note(this.newNote);
      await this.downloadNotes();
    },
    async downloadNotes() {
      this.verifyUser();
      this.notes = await blockchain_chat_backend.get_notes(this.identity!.getPrincipal());
    },
    async logIn(){
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      });

      const identity = authClient.getIdentity();
      console.log(`User logged in ${identity}`)
      this.identity = identity;
      this.principalName = identity.getPrincipal().toText();
      this.downloadNotes();
    }
  },
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div id="main-container">
      <div id="log-in">
        {{ principalName }}
        <button @click="logIn()">Log In</button>
      </div>
      <div id="notes" v-for="note, idx in notes[0]" :key="idx">
        <span>{{idx + 1 }}</span>: <span>{{ note }}</span>
      </div>
      <div id="add-note-container">
        <textarea v-model="newNote" placeholder="Add new note..."></textarea>
        <button @click="addNote()">Add new note</button>
      </div>
      <div>
        {{ newNote }}
      </div>
  </div>
  </main>
</template>