# Utility functions for cleaning up AI-generated responses
import re

def clean_ai_response(text):
    """
    Clean up AI-generated text by removing common formatting artifacts.
    Useful for commit messages and review outputs.
    """
    if not text:
        return ""
    
    # Remove markdown code blocks (```text```, ```python```, etc.)
    text = re.sub(r'```[\w]*\n?', '', text)
    text = re.sub(r'```', '', text)
    
    # Remove backticks around single words/phrases
    text = re.sub(r'`([^`]+)`', r'\1', text)
    
    # Remove quotes that wrap the entire response
    text = text.strip()
    if (text.startswith('"') and text.endswith('"')) or (text.startswith("'") and text.endswith("'")):
        text = text[1:-1]
    
    # Remove common AI prefixes/suffixes
    prefixes_to_remove = [
        r'^(here\'s?\s+(the\s+)?)?commit\s+message:?\s*',
        r'^(here\'s?\s+(a\s+)?)?review:?\s*',
        r'^(here\'s?\s+(the\s+)?)?analysis:?\s*',
        r'^response:?\s*',
        r'^answer:?\s*',
        r'^result:?\s*'
    ]
    
    for prefix in prefixes_to_remove:
        text = re.sub(prefix, '', text, flags=re.IGNORECASE)
    
    # Remove trailing explanations
    suffixes_to_remove = [
        r'\s*\n\n.*this\s+(commit\s+)?message.*$',
        r'\s*\n\n.*hope\s+this\s+helps.*$',
        r'\s*\n\n.*let\s+me\s+know.*$'
    ]
    
    for suffix in suffixes_to_remove:
        text = re.sub(suffix, '', text, flags=re.IGNORECASE | re.DOTALL)
    
    # Clean up extra whitespace
    text = re.sub(r'\n\s*\n\s*\n', '\n\n', text)  # Multiple blank lines -> single blank line
    text = text.strip()
    
    return text

def clean_commit_message(text):
    """
    Specifically clean commit messages.
    """
    text = clean_ai_response(text)
    
    # Remove commit-specific artifacts
    commit_artifacts = [
        r'^commit\s+message:?\s*',
        r'^git\s+commit\s+message:?\s*',
        r'^\s*-\s*commit:?\s*',
        r'^\s*\*\s*commit:?\s*',
        r'^here\s+is\s+the\s+commit\s+message:?\s*',
        r'^the\s+commit\s+message\s+is:?\s*'
    ]
    
    for artifact in commit_artifacts:
        text = re.sub(artifact, '', text, flags=re.IGNORECASE)
    
    # Remove quotes if they wrap a simple commit message
    text = text.strip()
    if text.count('"') == 2 and text.startswith('"') and text.endswith('"'):
        text = text[1:-1]
    elif text.count("'") == 2 and text.startswith("'") and text.endswith("'"):
        text = text[1:-1]
    
    # Ensure proper formatting for multi-line commits
    lines = text.split('\n')
    if len(lines) > 1:
        # First line should be the summary, followed by blank line, then details
        cleaned_lines = [lines[0].strip()]
        if len(lines) > 1 and lines[1].strip():
            cleaned_lines.append('')  # Add blank line if not present
        cleaned_lines.extend([line.rstrip() for line in lines[1:] if line.strip()])
        text = '\n'.join(cleaned_lines)
    
    return text.strip()

def clean_review_output(text):
    """
    Specifically clean review outputs while preserving formatting.
    """
    text = clean_ai_response(text)
    
    # Remove review-specific artifacts
    review_artifacts = [
        r'^code\s+review:?\s*',
        r'^review\s+results?:?\s*',
        r'^analysis\s+results?:?\s*'
    ]
    
    for artifact in review_artifacts:
        text = re.sub(artifact, '', text, flags=re.IGNORECASE)
    
    # Preserve important formatting like headers, lists, etc.
    # Just clean up excessive whitespace
    text = re.sub(r'\n{3,}', '\n\n', text)  # Max 2 consecutive newlines
    
    return text.strip()

def extract_main_content(text):
    """
    Extract the main content from AI responses that might have
    explanations before or after the actual content.
    """
    if not text:
        return ""
    
    lines = text.strip().split('\n')
    
    # Find the start of main content (skip intro lines)
    start_idx = 0
    intro_patterns = [
        r'here\'s?\s+(the\s+|a\s+)?',
        r'i\'ll\s+',
        r'let\s+me\s+',
        r'sure,?\s+',
        r'of\s+course,?\s+'
    ]
    
    for i, line in enumerate(lines):
        if any(re.match(pattern, line.strip(), re.IGNORECASE) for pattern in intro_patterns):
            start_idx = i + 1
            break
    
    # Find the end of main content (before explanations)
    end_idx = len(lines)
    outro_patterns = [
        r'this\s+(commit\s+)?message',
        r'hope\s+this\s+helps',
        r'let\s+me\s+know',
        r'feel\s+free\s+to',
        r'if\s+you\s+need'
    ]
    
    for i in range(start_idx, len(lines)):
        if any(re.search(pattern, lines[i].strip(), re.IGNORECASE) for pattern in outro_patterns):
            end_idx = i
            break
    
    # Extract and clean the main content
    main_content = '\n'.join(lines[start_idx:end_idx]).strip()
    return clean_ai_response(main_content)

def is_wrapped_in_quotes(text):
    """Check if text is wrapped in quotes and should be unwrapped."""
    text = text.strip()
    return ((text.startswith('"') and text.endswith('"')) or 
            (text.startswith("'") and text.endswith("'")))

def remove_markdown_artifacts(text):
    """Remove common markdown artifacts from AI responses."""
    # Remove markdown headers if they're just formatting
    text = re.sub(r'^#+\s*(commit message|review|analysis)\s*:?\s*$', '', text, flags=re.IGNORECASE | re.MULTILINE)
    
    # Remove bold/italic markdown
    text = re.sub(r'\*\*(.*?)\*\*', r'\1', text)
    text = re.sub(r'\*(.*?)\*', r'\1', text)
    
    return text
