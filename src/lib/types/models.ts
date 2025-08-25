export type MessageRole = 'user' | 'assistant' | 'system';

export interface ConversationMetadata {
  total_messages: number;
  total_tokens: number;
  last_model_used?: string | null;
  average_response_time?: number | null;
  tags?: string[];
  priority?: 'Low' | 'Normal' | 'High' | 'Critical';
}

export interface Conversation {
  id?: number | null;
  uuid: string;
  title: string;
  persona_id?: number | null;
  created_at: string; // ISO timestamp
  updated_at: string; // ISO timestamp
  archived: boolean;
  metadata?: ConversationMetadata | null;
}

export interface MessageMetadata {
  processing_time_ms?: number | null;
  confidence_score?: number | null;
  flagged_content?: boolean;
}

export interface Message {
  id?: number | null;
  conversation_id: number;
  role: MessageRole | string; // backend sends lowercase variants
  content: string;
  metadata?: MessageMetadata | null;
  created_at: string; // ISO timestamp
  tokens_used?: number | null;
  model_used?: string | null;
}

export interface Persona {
  id?: number | null;
  name: string;
  description?: string | null;
  system_prompt: string;
  avatar_path?: string | null;
  created_at: string;
  updated_at: string;
  active: boolean;
}

export interface AiResponse {
  content: string;
  model_used: string;
  tokens_used: number;
  processing_time_ms: number;
}
