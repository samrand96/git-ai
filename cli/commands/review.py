# AI-powered code review command for git-ai
import os
import argparse
import sys
from datetime import datetime
from core.config import settings
from providers.factory import get_provider
from utils import get_diff, clean_review_output, Colors, format_cli_output

REVIEW_TYPES = ["all", "logical", "security", "performance", "style", "documentation"]


def main():
    parser = argparse.ArgumentParser(description="Professional AI-powered code review for your changes.")
    parser.add_argument('--type', choices=REVIEW_TYPES, default="all", help='Type of review: all, logical, security, performance, style, or documentation')
    parser.add_argument('--html', action='store_true', help='Output review as HTML file (AI generates a beautiful HTML report)')
    parser.add_argument('--output', type=str, help='HTML output file name (default: ai_review_TIMESTAMP.html)')
    parser.add_argument('--severity', choices=['low', 'medium', 'high', 'critical'], default='medium', help='Minimum severity level to report (default: medium)')
    parser.add_argument('--changes', choices=['staged', 'unstaged', 'all', 'last-commit'], default='all', help='What changes to review: staged, unstaged, all, or last-commit (default: all)')
    args = parser.parse_args()

    provider_name = settings.get('PROVIDER', 'openai')
    provider = get_provider(provider_name)

    # Get the appropriate diff based on user choice
    changes_type = args.changes
    if changes_type == 'staged':
        diff = get_diff(staged=True)
        changes_desc = "staged changes"
    elif changes_type == 'unstaged':
        diff = get_diff(staged=False) 
        changes_desc = "unstaged changes"
    elif changes_type == 'last-commit':
        diff = get_diff(commit='HEAD~1..HEAD')
        changes_desc = "last commit changes"
    else:  # 'all'
        staged_diff = get_diff(staged=True)
        unstaged_diff = get_diff(staged=False)
        diff = f"{staged_diff}\n{unstaged_diff}".strip()
        changes_desc = "all changes (staged + unstaged)"

    if not diff.strip():
        print(Colors.info(f"ℹ No {changes_desc} to review."))
        print(Colors.warning("💡 Try:"))
        print(Colors.dim("  - Make some changes to your files"))
        print(Colors.dim("  - Use --changes staged (for git add'ed files)"))
        print(Colors.dim("  - Use --changes unstaged (for modified files)"))
        print(Colors.dim("  - Use --changes last-commit (for previous commit)"))
        return

    review_type = args.type
    html = args.html
    severity = args.severity
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_file = args.output or f"ai_review_{timestamp}.html"

    # Get repository context
    try:
        repo_files = []
        for root, dirs, files in os.walk('.'):
            # Skip common ignore patterns
            dirs[:] = [d for d in dirs if not d.startswith('.') and d not in ['node_modules', '__pycache__', 'build', 'dist']]
            for file in files:
                if file.endswith(('.py', '.js', '.ts', '.java', '.cpp', '.h', '.cs', '.go', '.rs', '.rb', '.php')):
                    repo_files.append(os.path.relpath(os.path.join(root, file)))
        repo_context = f"Repository files: {', '.join(repo_files[:20])}" + ("..." if len(repo_files) > 20 else "")
    except:
        repo_context = "Repository context unavailable"

    if html:
        prompt = f"""You are a senior software architect and security expert. Perform a comprehensive, professional code review.

**REVIEW CONFIGURATION:**
- Review Type: {review_type.upper()}
- Changes Reviewed: {changes_desc}
- Minimum Severity: {severity}
- Repository Context: {repo_context}
- Timestamp: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}

**REVIEW SCOPE:**
{_get_review_scope_instructions(review_type)}

**OUTPUT FORMAT REQUIREMENT:**
Generate a complete, professional HTML document with:
1. Modern CSS styling with responsive design
2. Executive summary with metrics and risk assessment
3. Color-coded severity levels (Critical=red, High=orange, Medium=yellow, Low=blue, Info=gray)
4. Interactive table of contents
5. Code syntax highlighting
6. Detailed findings with file/line references
7. Actionable recommendations with priority levels
8. Security risk matrix (if applicable)
9. Performance impact analysis (if applicable)
10. Compliance checklist (best practices, style guide adherence)

Use professional icons, charts, and visual elements. Make it presentation-ready for stakeholders.

**CODE CHANGES TO REVIEW:**
```diff
{diff}
```

Generate the complete HTML report now:"""
    else:
        prompt = f"""You are a senior software architect and security expert. Perform a comprehensive, professional code review.

**REVIEW CONFIGURATION:**
- Review Type: {review_type.upper()}
- Changes Reviewed: {changes_desc}
- Minimum Severity: {severity}
- Repository Context: {repo_context}

**REVIEW SCOPE:**
{_get_review_scope_instructions(review_type)}

**CLI OUTPUT FORMAT REQUIREMENTS:**
- Use clear section headers with ASCII art dividers
- Categorize findings by severity: CRITICAL, HIGH, MEDIUM, LOW, INFO
- For each finding, provide: file:line, severity, description, recommendation
- Use bullet points and sub-bullets for organization
- Include code snippets with line numbers when relevant
- Add emojis for visual clarity: 🔴 Critical, 🟠 High, 🟡 Medium, 🔵 Low, ℹ️ Info
- End with summary statistics and next steps
- Format for terminal display with appropriate line breaks
- Use consistent indentation and spacing
- Highlight important keywords with UPPERCASE when needed

**IMPORTANT CLI FORMATTING NOTES:**
- Mark severe issues with keywords like "ERROR", "VULNERABILITY", "CRITICAL", "DANGEROUS" so they get colored red
- Mark warnings with "WARNING", "CAUTION", "POTENTIAL", "CONSIDER", "SHOULD" so they get colored yellow
- Mark good practices with "GOOD", "EXCELLENT", "WELL", "NICE", "CORRECT", "PROPER" so they get colored green
- Use "##" for section headers to get cyan color formatting
- Use "-" or "*" for bullet points to get white color formatting

**CODE CHANGES TO REVIEW:**
```diff
{diff}
```

Provide your comprehensive review now:"""

    print(Colors.header(f"🔍 Reviewing your {changes_desc} with AI..."))
    try:
        review = provider.generate(prompt=prompt)
        # Clean up the AI-generated review
        review = clean_review_output(review)
    except Exception as e:
        print(Colors.error(f"❌ Error generating review: {e}"))
        return

    if html:
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(review)
        print(Colors.success(f"✅ Professional AI review report saved to: {output_file}"))
        print(Colors.info(f"📊 Open the HTML file in your browser to view the interactive report"))
    else:
        formatted_review = format_cli_output(review)
        print(Colors.header("\n" + "="*60))
        print(Colors.header("  🚀 PROFESSIONAL AI CODE REVIEW REPORT  "))
        print(Colors.header("="*60 + "\n"))
        print(formatted_review)
        print(Colors.header("\n" + "="*60))
        print(Colors.header("  📋 Review Complete - Check findings above  "))
        print(Colors.header("="*60))

def _get_review_scope_instructions(review_type):
    """Get detailed instructions for each review type"""
    scope_map = {
        "all": """
- Code Logic & Correctness: Algorithm efficiency, edge cases, error handling
- Security Vulnerabilities: SQL injection, XSS, authentication flaws, data exposure
- Performance Issues: Memory leaks, inefficient algorithms, database queries
- Code Quality: Readability, maintainability, SOLID principles, design patterns
- Documentation: Comments, docstrings, API documentation
- Style & Standards: Coding conventions, naming, formatting, best practices""",

        "logical": """
- Algorithm correctness and efficiency
- Edge case handling and boundary conditions  
- Error handling and exception management
- Data flow and state management
- Logic flaws and potential bugs
- Return value validation""",

        "security": """
- Input validation and sanitization
- Authentication and authorization flaws
- SQL injection and XSS vulnerabilities
- Data exposure and privacy issues
- Cryptographic implementations
- Access control mechanisms""",

        "performance": """
- Algorithm time and space complexity
- Database query optimization
- Memory management and leaks
- Caching strategies
- Resource utilization
- Scalability considerations""",

        "style": """
- Coding conventions and standards
- Naming conventions consistency
- Code formatting and structure
- Documentation completeness
- Design pattern usage
- Code organization""",
        "documentation": """
- Code comments quality and coverage
- API documentation completeness
- Function/method docstrings
- README and setup instructions
- Inline explanations for complex logic
- Examples and usage patterns"""
    }
    return scope_map.get(review_type, scope_map["all"])

if __name__ == "__main__":
    main()
