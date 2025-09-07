# Main entry point for git-ai CLI
import sys
from cli.parser import get_main_parser, COMMANDS

from pathlib import Path
from core.config import settings

def main():
    parser = get_main_parser()
    args, unknown = parser.parse_known_args()
    config_is_valid = settings.config_is_valid()
    if args.command != 'config' and (not config_is_valid):
        print("[ERROR] No configuration file found. Please run 'python main.py config --interactive' to set up git-ai before using this command.")
        cmd_func = COMMANDS['config']
        sys.argv = [sys.argv[0]] + unknown + ['--interactive']
        cmd_func()
        return
    cmd_func = COMMANDS[args.command]
    sys.argv = [sys.argv[0]] + unknown
    cmd_func()

if __name__ == "__main__":
    main()
