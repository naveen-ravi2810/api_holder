import { defineStore } from "#imports";
import { authState } from "./state";
import { authActions } from "./actions";

export const useAuthStore = defineStore("user", {
  state: authState,
  actions: authActions,
});
