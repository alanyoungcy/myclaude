import { invoke } from '@tauri-apps/api/core';

export interface AppConfig {
  api_key: string;
  base_url: string;
  model: string;
  system_prompt: string;
  tavily_api_key: string;
}

export interface Conversation {
  id: string;
  title: string;
  created_at: string;
}

export interface ChatMessage {
  id: string;
  conversation_id: string;
  role: string;
  content: string;
  created_at: string;
}

export interface SystemPrompt {
  id: string;
  name: string;
  prompt: string;
  created_at: string;
}

export interface SendMessageRequest {
  conversation_id: string;
  message: string;
  mode?: string;
}

export interface SendMessageResponse {
  message: ChatMessage;
  assistant_message: ChatMessage;
}

export interface ConversationWithMessages {
  conversation: Conversation;
  messages: ChatMessage[];
}

export interface WebSearchResult {
  title: string;
  url: string;
  content: string;
  score: number;
}

export interface Skill {
  name: string;
  description: string;
  instructions: string;
  parameters: any;
}

// Configuration
export const getConfig = (): Promise<AppConfig> => invoke('get_config');
export const updateConfig = (config: AppConfig): Promise<void> => invoke('update_config', { config });

// Models - returns array of model ID strings
export const getModels = (): Promise<string[]> => invoke('get_models');

// Conversations
export const getConversations = (): Promise<Conversation[]> => invoke('get_conversations');
export const getConversation = (id: string): Promise<ConversationWithMessages | null> => 
  invoke('get_conversation', { id });
export const createConversation = (title: string): Promise<Conversation> => invoke('create_conversation', { title });
export const deleteConversation = (id: string): Promise<void> => invoke('delete_conversation', { id });

// Messages
export const sendMessage = (request: SendMessageRequest): Promise<SendMessageResponse> => 
  invoke('send_message', { request });

// System Prompts
export const getSystemPrompts = (): Promise<SystemPrompt[]> => invoke('get_system_prompts');
export const saveSystemPrompt = (name: string, prompt: string): Promise<SystemPrompt> => 
  invoke('save_system_prompt', { name, prompt });
export const deleteSystemPrompt = (id: string): Promise<void> => invoke('delete_system_prompt', { id });

// Web Search
export const webSearch = (query: string, maxResults?: number): Promise<WebSearchResult[]> =>
  invoke('web_search', { query, maxResults });

// Skills
export const getSkills = (): Promise<Skill[]> => invoke('get_skills');
