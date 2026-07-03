import { useState, useEffect } from 'react';
import { useStore } from '../store';
import { getSystemPrompts, saveSystemPrompt, deleteSystemPrompt, updateConfig, type SystemPrompt } from '../api';

interface PromptsManagerProps {
  onClose: () => void;
}

export default function PromptsManager({ onClose }: PromptsManagerProps) {
  const { config, setConfig } = useStore();
  const [prompts, setPrompts] = useState<SystemPrompt[]>([]);
  const [newName, setNewName] = useState('');
  const [newPrompt, setNewPrompt] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadPrompts();
  }, []);

  const loadPrompts = async () => {
    try {
      const list = await getSystemPrompts();
      setPrompts(list);
    } catch (error) {
      console.error('Failed to load prompts:', error);
    }
  };

  const handleSave = async () => {
    if (!newName.trim() || !newPrompt.trim()) {
      alert('Please enter both name and prompt');
      return;
    }

    setLoading(true);
    try {
      await saveSystemPrompt(newName, newPrompt);
      setNewName('');
      setNewPrompt('');
      await loadPrompts();
    } catch (error) {
      alert('Failed to save prompt: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Delete this prompt?')) return;

    try {
      await deleteSystemPrompt(id);
      await loadPrompts();
    } catch (error) {
      alert('Failed to delete prompt: ' + error);
    }
  };

  const handleUse = async (prompt: string) => {
    if (!config) return;

    try {
      const updated = { ...config, system_prompt: prompt };
      await updateConfig(updated);
      setConfig(updated);
      alert('System prompt updated!');
      onClose();
    } catch (error) {
      alert('Failed to update config: ' + error);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-3xl max-h-[90vh] overflow-y-auto">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-bold">System Prompts</h2>
          <button onClick={onClose} className="text-gray-400 hover:text-white text-2xl">
            ×
          </button>
        </div>

        <div className="mb-6 p-4 bg-gray-700 rounded-lg">
          <h3 className="font-medium mb-3">Create New Prompt</h3>
          <div className="space-y-3">
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder="Prompt name (e.g., 'Coding Assistant')"
              className="w-full bg-gray-600 text-white rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <textarea
              value={newPrompt}
              onChange={(e) => setNewPrompt(e.target.value)}
              placeholder="Enter system prompt..."
              rows={4}
              className="w-full bg-gray-600 text-white rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button
              onClick={handleSave}
              disabled={loading}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded font-medium"
            >
              {loading ? 'Saving...' : 'Save Prompt'}
            </button>
          </div>
        </div>

        <div className="space-y-3">
          <h3 className="font-medium">Saved Prompts</h3>
          {prompts.length === 0 ? (
            <p className="text-gray-400 text-sm">No saved prompts yet</p>
          ) : (
            prompts.map((prompt) => (
              <div key={prompt.id} className="bg-gray-700 rounded-lg p-4">
                <div className="flex justify-between items-start mb-2">
                  <h4 className="font-medium">{prompt.name}</h4>
                  <div className="flex space-x-2">
                    <button
                      onClick={() => handleUse(prompt.prompt)}
                      className="px-3 py-1 bg-green-600 hover:bg-green-700 rounded text-sm"
                    >
                      Use
                    </button>
                    <button
                      onClick={() => handleDelete(prompt.id)}
                      className="px-3 py-1 bg-red-600 hover:bg-red-700 rounded text-sm"
                    >
                      Delete
                    </button>
                  </div>
                </div>
                <p className="text-sm text-gray-300 whitespace-pre-wrap">{prompt.prompt}</p>
                <p className="text-xs text-gray-500 mt-2">
                  Created: {new Date(prompt.created_at).toLocaleString()}
                </p>
              </div>
            ))
          )}
        </div>

        <div className="flex justify-end mt-6">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded font-medium"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
