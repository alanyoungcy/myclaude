import { useState, useEffect } from 'react';
import { getSkills, Skill } from '../api';
import Canvas from './Canvas';

interface SkillsManagerProps {
  onClose: () => void;
}

export default function SkillsManager({ onClose }: SkillsManagerProps) {
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedSkill, setSelectedSkill] = useState<Skill | null>(null);

  useEffect(() => {
    loadSkills();
  }, []);

  const loadSkills = async () => {
    try {
      setLoading(true);
      const loadedSkills = await getSkills();
      setSkills(loadedSkills);
    } catch (error) {
      console.error('Failed to load skills:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-900 rounded-lg w-full max-w-5xl max-h-[90vh] flex flex-col border border-gray-700 shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-700">
          <div>
            <h2 className="text-2xl font-bold text-white">Skills Manager</h2>
            <p className="text-sm text-gray-400 mt-1">
              Manage and view available skills (.md files)
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden flex">
          {/* Skills List */}
          <div className="w-80 border-r border-gray-700 overflow-y-auto">
            <div className="p-4 space-y-2">
              {loading ? (
                <div className="text-center text-gray-400 py-8">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
                  <p className="mt-2">Loading skills...</p>
                </div>
              ) : skills.length === 0 ? (
                <div className="text-center text-gray-400 py-8">
                  <svg className="w-12 h-12 mx-auto mb-3 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  <p className="font-medium mb-1">No skills found</p>
                  <p className="text-sm">Add .md files to the skills directory</p>
                </div>
              ) : (
                skills.map((skill) => (
                  <button
                    key={skill.name}
                    onClick={() => setSelectedSkill(skill)}
                    className={`w-full text-left p-3 rounded-lg transition-colors ${
                      selectedSkill?.name === skill.name
                        ? 'bg-blue-600 text-white'
                        : 'bg-gray-800 text-gray-200 hover:bg-gray-700'
                    }`}
                  >
                    <div className="font-medium">{skill.name}</div>
                    <div className="text-sm text-gray-400 mt-1 line-clamp-2">
                      {skill.description}
                    </div>
                  </button>
                ))
              )}
            </div>
          </div>

          {/* Skill Detail */}
          <div className="flex-1 overflow-y-auto p-6">
            {selectedSkill ? (
              <div className="space-y-4">
                <div>
                  <h3 className="text-xl font-bold text-white mb-2">{selectedSkill.name}</h3>
                  <p className="text-gray-300">{selectedSkill.description}</p>
                </div>

                {selectedSkill.parameters && Object.keys(selectedSkill.parameters.properties || {}).length > 0 && (
                  <div>
                    <h4 className="text-lg font-semibold text-white mb-2">Parameters</h4>
                    <div className="bg-gray-800 rounded-lg p-4 border border-gray-700">
                      {Object.entries(selectedSkill.parameters.properties || {}).map(([key, value]: [string, any]) => (
                        <div key={key} className="mb-3 last:mb-0">
                          <div className="flex items-center space-x-2">
                            <span className="font-mono text-blue-400">{key}</span>
                            <span className="text-xs px-2 py-0.5 bg-gray-700 rounded text-gray-300">
                              {value.type}
                            </span>
                            {selectedSkill.parameters.required?.includes(key) && (
                              <span className="text-xs px-2 py-0.5 bg-red-900 text-red-200 rounded">
                                required
                              </span>
                            )}
                          </div>
                          <p className="text-sm text-gray-400 mt-1">{value.description}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                <div>
                  <h4 className="text-lg font-semibold text-white mb-3">Instructions</h4>
                  <Canvas
                    content={selectedSkill.instructions}
                    title={`${selectedSkill.name} Instructions`}
                    type="markdown"
                  />
                </div>
              </div>
            ) : (
              <div className="flex items-center justify-center h-full text-gray-500">
                <div className="text-center">
                  <svg className="w-16 h-16 mx-auto mb-4 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  <p className="text-lg">Select a skill to view details</p>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="border-t border-gray-700 p-4 bg-gray-900">
          <div className="flex items-center justify-between">
            <div className="text-sm text-gray-400">
              {skills.length} skill{skills.length !== 1 ? 's' : ''} available
            </div>
            <button
              onClick={onClose}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
            >
              Close
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
