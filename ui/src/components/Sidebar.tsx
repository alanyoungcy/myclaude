import { useStore } from '../store';
import { getConversations, createConversation, deleteConversation, getConversation } from '../api';

interface SidebarProps {
  onOpenSettings: () => void;
  onOpenPrompts: () => void;
  onOpenSkills: () => void;
}

export default function Sidebar({ onOpenSettings, onOpenPrompts, onOpenSkills }: SidebarProps) {
  const { conversations, setConversations, currentConversation, setCurrentConversation, setMessages } = useStore();

  const handleNewChat = async () => {
    try {
      const newConv = await createConversation('New Chat');
      setConversations([newConv, ...conversations]);
      setCurrentConversation(newConv);
      setMessages([]);
    } catch (error) {
      console.error('Failed to create conversation:', error);
    }
  };

  const handleSelectConversation = async (id: string) => {
    try {
      const conv = await getConversation(id);
      if (conv) {
        setCurrentConversation(conv.conversation);
        setMessages(conv.messages);
      }
    } catch (error) {
      console.error('Failed to load conversation:', error);
    }
  };

  const handleDeleteConversation = async (id: string) => {
    try {
      await deleteConversation(id);
      const updated = await getConversations();
      setConversations(updated);

      if (currentConversation?.id === id) {
        if (updated.length > 0) {
          handleSelectConversation(updated[0].id);
        } else {
          const newConv = await createConversation('New Chat');
          setConversations([newConv]);
          setCurrentConversation(newConv);
          setMessages([]);
        }
      }
    } catch (error) {
      console.error('Failed to delete conversation:', error);
    }
  };

  return (
    <div className="w-64 bg-surface flex flex-col border-r border-border">
      {/* Header */}
      <div className="p-4 border-b border-border">
        <button
          onClick={handleNewChat}
          className="btn btn-primary w-full"
        >
          <span className="text-lg mr-2">+</span> New Chat
        </button>
      </div>

      {/* Conversations list */}
      <div className="flex-1 overflow-y-auto p-2">
        {conversations.map((conv) => (
          <div
            key={conv.id}
            className={`group relative p-3 mb-2 rounded-lg cursor-pointer transition-all ${
              currentConversation?.id === conv.id
                ? 'bg-primary text-white'
                : 'hover:bg-background-secondary'
            }`}
            onClick={() => handleSelectConversation(conv.id)}
          >
            <div className="text-sm font-medium truncate">{conv.title}</div>
            <div className={`text-xs mt-1 ${
              currentConversation?.id === conv.id
                ? 'text-white opacity-80'
                : 'text-text-tertiary'
            }`}>
              {new Date(conv.created_at).toLocaleDateString()}
            </div>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleDeleteConversation(conv.id);
              }}
              className={`absolute right-2 top-3 opacity-0 group-hover:opacity-100 transition-opacity ${
                currentConversation?.id === conv.id
                  ? 'text-white hover:text-red-200'
                  : 'text-error hover:text-error'
              }`}
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        ))}
      </div>

      {/* Footer menu */}
      <div className="p-4 border-t border-border space-y-2">
        <button
          onClick={onOpenSkills}
          className="btn btn-ghost w-full justify-start text-sm"
        >
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
          </svg>
          Skills
        </button>
        <button
          onClick={onOpenPrompts}
          className="btn btn-ghost w-full justify-start text-sm"
        >
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          Manage Prompts
        </button>
        <button
          onClick={onOpenSettings}
          className="btn btn-ghost w-full justify-start text-sm"
        >
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          Settings
        </button>
      </div>
    </div>
  );
}
