# Color utilities for git-ai CLI
import os
import sys

class Colors:
    """ANSI color codes and utilities for terminal output"""
    
    # Color codes
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    WHITE = '\033[97m'
    GRAY = '\033[90m'
    
    # Style codes
    BOLD = '\033[1m'
    DIM = '\033[2m'
    ITALIC = '\033[3m'
    UNDERLINE = '\033[4m'
    BLINK = '\033[5m'
    REVERSE = '\033[7m'
    STRIKETHROUGH = '\033[9m'
    
    # Reset
    END = '\033[0m'
    RESET = '\033[0m'
    
    @staticmethod
    def supports_color():
        """Check if terminal supports ANSI colors"""
        # Check if we're in a terminal that supports colors
        if not hasattr(sys.stdout, 'isatty') or not sys.stdout.isatty():
            return False
        
        # Windows terminal support
        if os.name == 'nt':
            return 'ANSICON' in os.environ or 'WT_SESSION' in os.environ or os.environ.get('TERM_PROGRAM') == 'vscode'
        
        # Unix-like systems
        term = os.environ.get('TERM', '')
        return term != 'dumb' and term != ''
    
    @classmethod
    def colorize(cls, text, color):
        """Apply color to text if colors are supported"""
        if cls.supports_color():
            return f"{color}{text}{cls.END}"
        return text
    
    @classmethod
    def success(cls, text):
        """Green text for success messages"""
        return cls.colorize(text, cls.GREEN + cls.BOLD)
    
    @classmethod
    def error(cls, text):
        """Red text for error messages"""
        return cls.colorize(text, cls.RED + cls.BOLD)
    
    @classmethod
    def warning(cls, text):
        """Yellow text for warning messages"""
        return cls.colorize(text, cls.YELLOW + cls.BOLD)
    
    @classmethod
    def info(cls, text):
        """Blue text for info messages"""
        return cls.colorize(text, cls.BLUE + cls.BOLD)
    
    @classmethod
    def header(cls, text):
        """Cyan text for headers"""
        return cls.colorize(text, cls.CYAN + cls.BOLD)
    
    @classmethod
    def dim(cls, text):
        """Dim gray text for secondary info"""
        return cls.colorize(text, cls.GRAY + cls.DIM)
    
    @classmethod
    def highlight(cls, text):
        """Magenta text for highlighting"""
        return cls.colorize(text, cls.MAGENTA + cls.BOLD)

def format_cli_output(text):
    """
    Format text with colors based on content keywords.
    Used for AI-generated output like reviews and commit messages.
    """
    if not Colors.supports_color():
        return text
    
    lines = text.split('\n')
    formatted_lines = []
    
    for line in lines:
        line_lower = line.lower()
        
        # Critical issues - Red
        if any(word in line_lower for word in ['error', 'bug', 'vulnerability', 'critical', 'severe', 'dangerous', 'fail']):
            formatted_lines.append(Colors.colorize(line, Colors.RED + Colors.BOLD))
        
        # Warnings - Yellow
        elif any(word in line_lower for word in ['warning', 'caution', 'potential', 'consider', 'should', 'might']):
            formatted_lines.append(Colors.colorize(line, Colors.YELLOW))
        
        # Good practices - Green
        elif any(word in line_lower for word in ['good', 'excellent', 'well', 'nice', 'correct', 'proper', 'success']):
            formatted_lines.append(Colors.colorize(line, Colors.GREEN))
        
        # Headers (lines starting with #)
        elif line.startswith('#') or line.startswith('##'):
            formatted_lines.append(Colors.colorize(line, Colors.CYAN + Colors.BOLD))
        
        # List items
        elif line.strip().startswith(('-', '*', '•', '→')):
            formatted_lines.append(Colors.colorize(line, Colors.WHITE))
        
        # Numbers or bullet points at start
        elif line.strip() and (line.strip()[0].isdigit() or line.strip().startswith('○')):
            formatted_lines.append(Colors.colorize(line, Colors.WHITE))
        
        # Default
        else:
            formatted_lines.append(line)
    
    return '\n'.join(formatted_lines)

def print_section_header(title):
    """Print a formatted section header"""
    separator = "=" * max(60, len(title) + 4)
    print(Colors.header(separator))
    print(Colors.header(f"  {title}  "))
    print(Colors.header(separator))

def print_box(text, color=None):
    """Print text in a colored box"""
    lines = text.split('\n')
    max_width = max(len(line) for line in lines) + 4
    
    color_func = Colors.colorize if color else lambda x, c: x
    border_color = color or Colors.CYAN
    
    print(color_func("┌" + "─" * (max_width - 2) + "┐", border_color))
    for line in lines:
        padded_line = f"│ {line:<{max_width - 4}} │"
        print(color_func(padded_line, border_color))
    print(color_func("└" + "─" * (max_width - 2) + "┘", border_color))

def print_progress(message):
    """Print a progress message with spinner"""
    print(Colors.info(f"🔄 {message}"))

def print_step(step_num, total_steps, message):
    """Print a numbered step"""
    step_text = f"[{step_num}/{total_steps}]"
    print(Colors.dim(step_text) + " " + Colors.info(message))
