# Centralized argument parser for git-ai main CLI
import argparse
from cli.commands import commit_main, config_main, list_models_main, review_main

COMMANDS = {
    'commit': commit_main,
    'config': config_main,
    'list-models': list_models_main,
    'review': review_main,
}

def get_main_parser():
    parser = argparse.ArgumentParser(
        description="git-ai: AI-powered Git commit assistant"
    )
    parser.add_argument('command', choices=COMMANDS.keys(), help='Command to run')
    return parser
