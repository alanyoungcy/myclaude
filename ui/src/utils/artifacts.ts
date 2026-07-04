export interface Artifact {
  id: string;
  title: string;
  content: string;
  type: 'markdown' | 'code';
  language?: string;
  timestamp: number;
}

export function extractArtifacts(text: string): Artifact[] {
  const artifacts: Artifact[] = [];

  // Match code blocks with language identifier
  const codeBlockRegex = /```(\w+)?\n([\s\S]*?)```/g;
  let match;
  let index = 0;

  while ((match = codeBlockRegex.exec(text)) !== null) {
    const language = match[1] || 'text';
    const content = match[2].trim();

    // Only create artifact for substantial code blocks (more than 5 lines or 100 chars)
    if (content.split('\n').length > 5 || content.length > 100) {
      artifacts.push({
        id: `artifact-${Date.now()}-${index}`,
        title: `${language.charAt(0).toUpperCase() + language.slice(1)} Code`,
        content: content,
        type: 'code',
        language: language,
        timestamp: Date.now()
      });
      index++;
    }
  }

  // Check for markdown documents (substantial formatted content)
  const hasHeaders = /^#{1,6}\s+.+$/m.test(text);
  const hasList = /^[-*+]\s+.+$/m.test(text);
  const hasMultipleParagraphs = text.split('\n\n').length > 3;

  if ((hasHeaders || hasList) && hasMultipleParagraphs && artifacts.length === 0) {
    // This looks like a substantial markdown document
    artifacts.push({
      id: `artifact-${Date.now()}-md`,
      title: 'Document',
      content: text,
      type: 'markdown',
      timestamp: Date.now()
    });
  }

  return artifacts;
}

export function shouldShowInArtifacts(text: string): boolean {
  const artifacts = extractArtifacts(text);
  return artifacts.length > 0;
}

export function getArtifactTitle(content: string, type: 'markdown' | 'code', language?: string): string {
  if (type === 'code') {
    return `${language?.charAt(0).toUpperCase()}${language?.slice(1)} Code`;
  }

  // Try to extract title from markdown
  const firstHeader = content.match(/^#\s+(.+)$/m);
  if (firstHeader) {
    return firstHeader[1];
  }

  return 'Document';
}
