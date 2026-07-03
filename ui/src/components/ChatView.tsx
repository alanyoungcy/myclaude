import { useState, useRef, useEffect } from 'react';
import { useStore } from '../store';
import { sendMessage } from '../api';
import MessageCanvas from './MessageCanvas';

export default function ChatView() {
  const { currentConversation, messages, addMessage, isLoading, setLoading, config } = useStore();
  const [input, setInput] = useState('');
  const [dragOver, setDragOver] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || !currentConversation || isLoading) return;

    const userMessage = input.trim();
    setInput('');
    setLoading(true);

    try {
      const response = await sendMessage({
        conversation_id: currentConversation.id,
        message: userMessage,
      });

      addMessage(response.message);
      addMessage(response.assistant_message);
    } catch (error) {
      console.error('Failed to send message:', error);
      alert('Failed to send message: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    
    const files = Array.from(e.dataTransfer.files);
    console.log('Files dropped:', files);
    // TODO: Implement file upload
    alert('File upload coming soon!');
  };

  if (!config?.api_key || !config?.base_url) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-900">
        <div className="text-center">
          <h2 className="text-2xl font-bold mb-4">Welcome to MyClaude</h2>
          <p className="text-gray-400 mb-4">Please configure your API settings to get started.</p>
          <p className="text-sm text-gray-500">Click Settings in the sidebar to configure.</p>
        </div>
      </div>
    );
  }

  return (
    <div
      className="flex-1 flex flex-col bg-gray-900"
      onDragOver={(e) => {
        e.preventDefault();
        setDragOver(true);
      }}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
    >
      {dragOver && (
        <div className="absolute inset-0 bg-blue-500 bg-opacity-20 border-4 border-blue-500 border-dashed flex items-center justify-center z-10">
          <div className="text-2xl font-bold">Drop files here</div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 && (
          <div className="text-center text-gray-500 mt-20">
            <p className="text-xl">Start a conversation</p>
            <p className="text-sm mt-2">Claude can search the web when needed</p>
          </div>
        )}

        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div className="max-w-3xl w-full">
              <MessageCanvas content={msg.content} role={msg.role as 'user' | 'assistant'} />
            </div>
          </div>
        ))}

        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-gray-800 rounded-lg p-4">
              <div className="flex space-x-2">
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      <div className="border-t border-gray-700 p-4">
        <div className="flex space-x-2">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Type your message... (Shift+Enter for new line)"
            className="flex-1 bg-gray-800 text-white rounded-lg p-3 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
            rows={3}
            disabled={isLoading}
          />
          <button
            onClick={handleSend}
            disabled={!input.trim() || isLoading}
            className="px-6 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded-lg font-medium"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
