Git-AI: Auto-Generate Git Commit Messages with AI

Git-AI is a Python-based CLI tool that leverages OpenAI’s GPT models to generate professional and concise Git commit messages based on staged changes. It saves developers time and ensures clear, meaningful commit messages.

Features
	•	Analyze Staged Changes: Automatically reads your staged changes (git diff --staged) to understand what has been modified.
	•	AI-Generated Commit Messages: Generates contextually appropriate commit messages using OpenAI’s GPT models.
	•	Customizable Output: View the suggested commit message or directly apply it.
	•	Cross-Platform: Works seamlessly on macOS, Linux, and Windows.

Requirements
	•	Python 3.7 or higher
	•	Git installed and configured
	•	OpenAI API key

Installation
	1.	Clone the repository:

git clone https://github.com/your-username/git-ai.git
cd git-ai


	2.	Install dependencies:

pip install -r requirements.txt


	3.	Set up your OpenAI API key:
	•	Create a .env file in the project directory:

OPENAI_API_KEY=your_openai_api_key

Usage

Generate a Commit Message
	1.	Stage your changes in Git:

git add .


	2.	Run Git-AI to generate a commit message:

python git-ai.py

Example output:

Suggested commit message:
- Fix null pointer exception in user authentication
- Update README with installation steps


	3.	Use the message manually, or proceed with the following step to commit automatically.

Automatically Commit

Run Git-AI with the --commit flag to generate and apply the commit message:

python git-ai.py --commit

You will be prompted to confirm the commit message before applying it.

How It Works
	1.	Git Integration:
	•	Fetches staged changes using git diff --staged.
	2.	AI-Powered Suggestions:
	•	Sends the diff summary to OpenAI’s GPT model to generate a professional commit message.
	3.	Command-Line Interface:
	•	Offers options to preview or directly apply the generated commit message.

Creating a Binary (macOS)
	1.	Install PyInstaller:

pip install pyinstaller


	2.	Build the executable:

pyinstaller --onefile git-ai.py


	3.	The executable will be located in the dist/ directory. Move it to your system path for global usage:

mv dist/git-ai /usr/local/bin/git-ai
chmod +x /usr/local/bin/git-ai


	4.	Run the tool globally:

git-ai --commit

Examples

Suggested Commit Message

python git-ai.py

Output:

Suggested commit message:
- Update user authentication logic
- Fix typo in error message

Auto-Commit

python git-ai.py --commit

Output:

Generating commit message...
Suggested commit message:
- Refactor database connection logic

Do you want to use this commit message? [y/N]: y
Commit successfully applied!

Configuration

Environment Variables

The script reads the OpenAI API key from the .env file or environment variables. Ensure the variable is set:

export OPENAI_API_KEY=your_openai_api_key

Contribution

Contributions are welcome! To contribute:
	1.	Fork the repository.
	2.	Create a feature branch:

git checkout -b feature-name


	3.	Commit your changes:

git commit -m "Add new feature"


	4.	Push the branch:

git push origin feature-name


	5.	Open a pull request.

License

This project is licensed under the MIT License. See the LICENSE file for details.

Acknowledgments
	•	OpenAI for their GPT API
	•	Python community for libraries like dotenv, argparse, and subprocess

Enjoy using Git-AI! 🚀
