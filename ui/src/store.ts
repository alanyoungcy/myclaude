import { create } from 'zustand';
import type { Conversation, ChatMessage, AppConfig } from './api';

export type ChatMode = 'general' | 'code' | 'research' | 'write';

interface AppState {
  config: AppConfig | null;
  conversations: Conversation[];
  currentConversation: Conversation | null;
  messages: ChatMessage[];
  isLoading: boolean;
  error: string | null;
  chatMode: ChatMode;

  setConfig: (config: AppConfig) => void;
  setConversations: (conversations: Conversation[]) => void;
  setCurrentConversation: (conversation: Conversation | null) => void;
  setMessages: (messages: ChatMessage[]) => void;
  addMessage: (message: ChatMessage) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setChatMode: (mode: ChatMode) => void;
}

export const useStore = create<AppState>((set) => ({
  config: null,
  conversations: [],
  currentConversation: null,
  messages: [],
  isLoading: false,
  error: null,
  chatMode: 'code',

  setConfig: (config) => set({ config }),
  setConversations: (conversations) => set({ conversations }),
  setCurrentConversation: (conversation) => set({ currentConversation: conversation }),
  setMessages: (messages) => set({ messages }),
  addMessage: (message) => set((state) => ({ messages: [...state.messages, message] })),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
  setChatMode: (mode) => set({ chatMode: mode }),
}));
