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
    <div className="w-64 bg-gray-800 flex flex-col border-r border-gray-700">
      <div className="p-4 border-b border-gray-700">
        <button
          onClick={handleNewChat}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded"
        >
          + New Chat
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-2">
        {conversations.map((conv) => (
          <div
            key={conv.id}
            className={`group relative p-3 mb-2 rounded cursor-pointer hover:bg-gray-700 ${
              currentConversation?.id === conv.id ? 'bg-gray-700' : ''
            }`}
            onClick={() => handleSelectConversation(conv.id)}
          >
            <div className="text-sm truncate">{conv.title}</div>
            <div className="text-xs text-gray-400 mt-1">
              {new Date(conv.created_at).toLocaleDateString()}
            </div>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleDeleteConversation(conv.id);
              }}
              className="absolute right-2 top-3 opacity-0 group-hover:opacity-100 text-red-400 hover:text-red-300"
            >
              ✕
            </button>
          </div>
        ))}
      </div>

      <div className="p-4 border-t border-gray-700 space-y-2">
        <button
          onClick={onOpenSkills}
          className="w-full text-left px-3 py-2 rounded hover:bg-gray-700 text-sm"
        >
          🛠️ Skills
        </button>
        <button
          onClick={onOpenPrompts}
          className="w-full text-left px-3 py-2 rounded hover:bg-gray-700 text-sm"
        >
          📝 Manage Prompts
        </button>
        <button
          onClick={onOpenSettings}
          className="w-full text-left px-3 py-2 rounded hover:bg-gray-700 text-sm"
        >
          ⚙️ Settings
        </button>
      </div>
    </div>
  );
}
