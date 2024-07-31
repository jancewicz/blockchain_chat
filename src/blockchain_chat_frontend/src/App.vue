<script lang="ts">
import { blockchain_chat_backend, canisterId, createActor } from '../../declarations/blockchain_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import type { Principal as PrincipalType } from '@dfinity/principal';
import type { UserData } from '../../declarations/blockchain_chat_backend/blockchain_chat_backend.did';

export default {
  data() {
    return {
      newMessage: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal: undefined as undefined | PrincipalType,
      targetPrincipal: "",
      userData: undefined as undefined | UserData,
      newNickname: "",
    }
  },
  methods: {
    isLogged(){
      if(!this.identity || !this.principal ||this.principal === Principal.anonymous()) {
        throw new Error("User is not logged in");
      }
      return {
        identity: this.identity,
        principal: this.principal
      }
    },
    validateTargetPrincipal(){
      const trimTargetPrincipal = this.targetPrincipal.trim()
      if(trimTargetPrincipal === "" ) {
        throw new Error("Principal not given");
      }
      const targetPrincipal = Principal.fromText(trimTargetPrincipal);
      if(!targetPrincipal || targetPrincipal === Principal.anonymous()) {
        throw new Error("Wrong target");
      }
      return targetPrincipal;
    },
    getAuthClient(){
      this.isLogged();
      return createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }});
    },
    async addChatMessage() {
      const targetPrincipal = this.validateTargetPrincipal();
      const backend = this.getAuthClient();
      
      await backend.add_chat_message(this.newMessage, targetPrincipal);
      await this.downloadChatMessages();
    },
    async downloadChatMessages() {
      const {identity, principal} = this.isLogged();
      const targetPrincipal = this.validateTargetPrincipal();
      
      const chatPath = [identity.getPrincipal(), targetPrincipal].sort();
      this.chats = await blockchain_chat_backend.get_chat(chatPath);
    },
    async getUserData() {
      const {identity, principal} = this.isLogged();
      const getUserData = await blockchain_chat_backend.get_user(principal as Principal);
      !getUserData.length ? this.userData = undefined : this.userData = getUserData[0];
    },
    async logIn(){
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/",
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          const principal = identity.getPrincipal();
          this.principal = principal;
          this.identity = identity;

          await this.getUserData();
        }
      })
    },
    async logOut() {
      const authClient = await AuthClient.create();
      await authClient.logout();
      this.identity = undefined;
      this.principal = undefined;
      this.chats = [];
      this.userData = undefined;
    },
    async registerNickname() {
      const trimNickname = this.newNickname.trim();
      const backend = this.getAuthClient();
      await backend.register(trimNickname);
      await this.getUserData();
    },
  },
}
</script>

<template>
  <main>
    <div id="main-container">
      <div id="log-in">
        {{ principal }}
        <button v-if="!principal" @click="logIn">Log In</button>
        <button v-if="principal" @click="logOut">Log Out</button>
        <div v-if="principal && !userData">
          <input v-model="newNickname" placeholder="Enter new nickname">
         <button @click="registerNickname()">Register new nickname</button>
        </div>
      </div>
      <div v-if="principal && userData">
        <div>
          <input v-model="targetPrincipal"  placeholder="download chat"><button @click="downloadChatMessages">Get chat</button>
        </div>
        <div id="notes" v-for="chat, idx in chats[0]" :key="idx">
          <span>{{idx + 1 }}</span>: <span>{{ chat }}</span>
        </div>
        <div id="add-note-container">
          <textarea v-model="newMessage" placeholder="Add new message..."></textarea>
          <button @click="addChatMessage()">Add new message</button>
        </div>
        <div>
          {{ newMessage }}
        </div>
      </div>
      
  </div>
  </main>
</template>