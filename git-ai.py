#!/usr/bin/env python3
import os
import platform
import argparse
import sys
import subprocess
import configparser
import re
from pathlib import Path
from openai import OpenAI

def get_config_dir():
    xdg = os.environ.get("XDG_CONFIG_HOME")
    if xdg:
        return Path(xdg)
    system = platform.system()
    home = Path.home()
    if system == 'Windows':
        return Path(os.getenv('APPDATA', home))
    if system == 'Darwin':
        return home / '.config'
    return home / '.config'

CONFIG_DIR = get_config_dir()
CONFIG_FILE = CONFIG_DIR / 'samrand.git.ini'


def create_config(default_format=None):
    CONFIG_DIR.mkdir(parents=True, exist_ok=True)
    cfg = configparser.ConfigParser()
    api_key = input(f"Enter OpenAI API key: ").strip()
    client = OpenAI(api_key=api_key)
    try:
        models = [m.id for m in client.models.list().data]
        for idx, m in enumerate(models, 1): print(f"{idx}. {m}")
        choice = input(f"Select a model (1-{len(models)}) or Enter for gpt-4: ").strip()
        model = models[int(choice)-1] if choice.isdigit() and 1 <= int(choice) <= len(models) else 'gpt-4'
    except:
        model = 'gpt-4'
    fmt = default_format or input("Default commit format ('detailed' or 'one-line') [detailed]: ").strip() or 'detailed'
    cfg['DEFAULT'] = {'API_KEY': api_key, 'MODEL': model, 'COMMIT_FORMAT': fmt}
    with open(CONFIG_FILE, 'w') as f: cfg.write(f)
    print(f"Configuration saved to {CONFIG_FILE}")
    return api_key, model, fmt


def load_config():
    cfg = configparser.ConfigParser()
    if not CONFIG_FILE.exists():
        return create_config('detailed')
    cfg.read(CONFIG_FILE)
    api_key = cfg['DEFAULT'].get('API_KEY')
    model = cfg['DEFAULT'].get('MODEL', 'gpt-4')
    fmt = cfg['DEFAULT'].get('COMMIT_FORMAT', 'detailed')
    if not api_key:
        return create_config(fmt)
    return api_key, model, fmt


def edit_config():
    api_key, model, fmt = load_config()
    print("Current configuration:")
    print(f"1. API key     : {api_key}")
    print(f"2. Model       : {model}")
    print(f"3. Commit format: {fmt}")
    print("Press Enter to keep current value.")
    new_key = input("New API key: ").strip() or api_key
    client = OpenAI(api_key=new_key)
    try:
        models = [m.id for m in client.models.list().data]
        for idx, m in enumerate(models, 1): print(f"{idx}. {m}")
        choice = input(f"Select model (1-{len(models)}) or Enter to keep '{model}': ").strip()
        new_model = models[int(choice)-1] if choice.isdigit() and 1 <= int(choice) <= len(models) else model
    except:
        new_model = model
    new_fmt = input(f"Commit format ('detailed' or 'one-line') [{fmt}]: ").strip() or fmt
    cfg = configparser.ConfigParser()
    cfg['DEFAULT'] = {'API_KEY': new_key, 'MODEL': new_model, 'COMMIT_FORMAT': new_fmt}
    with open(CONFIG_FILE, 'w') as f: cfg.write(f)
    print("Configuration updated.")
    sys.exit(0)


API_KEY, MODEL, COMMIT_FORMAT = load_config()
client = OpenAI(api_key=API_KEY)


def get_branch():
    try:
        return subprocess.check_output(['git','rev-parse','--abbrev-ref','HEAD'], text=True).strip()
    except:
        return ''


def prefix_from(branch):
    m = re.match(r'^([A-Za-z]+-\d+)', branch)
    return m.group(1) if m else ''


def stage():
    subprocess.run(['git','add','.'], check=True)


def diff():
    d = subprocess.check_output(['git','diff','--staged'], text=True)
    if not d.strip(): sys.exit(0)
    return d


def gen_msg(d):
    short = COMMIT_FORMAT == 'one-line'
    branch = get_branch()
    pref = prefix_from(branch)
    sys_msg = ("You are an expert Git assistant. Reply strictly with the commit message only."
               "If there was a ticket in the beginning of the branch write it as this TICKET_NUMBER: in the beginning of the commit message fill the TICKET_NUMBER with real one, otherwise leave it."
               "Prioritize those action that is mentioned in the branch name then the other action while writing the commit message, humanistic sound professional programmer with easy to read")
    user_msg = f"Branch: {branch}\nWrite a {'one-line' if short else 'detailed, human-friendly'} commit message for these changes:\n\n{d}"
    r = client.chat.completions.create(model=MODEL, messages=[{'role':'system','content':sys_msg},{'role':'user','content':user_msg}])
    text = r.choices[0].message.content.strip()
    if pref and not text.startswith(pref):
        text = f"{pref}: {text}"
    return text


def commit(msg):
    subprocess.run(['git','commit','-m', msg], check=True)


def save_fmt(fmt):
    cfg = configparser.ConfigParser(); cfg.read(CONFIG_FILE)
    cfg['DEFAULT']['COMMIT_FORMAT'] = fmt
    with open(CONFIG_FILE,'w') as f: cfg.write(f)


def push():
    if input("\nPush changes? [Y/n]: ").strip().lower() != 'n':
        subprocess.run(['git','push'], check=True)


def edit_inline(msg):
    print("Current message:\n", msg)
    if input("Edit before commit? [y/N]: ").strip().lower() == 'y':
        print("Enter new commit message. End with an empty line:")
        lines = []
        for line in sys.stdin:
            if line.strip() == '': break
            lines.append(line.rstrip('\n'))
        return '\n'.join(lines) if lines else msg
    return msg


def main():
    p = argparse.ArgumentParser()
    p.add_argument('-l','--list-models',action='store_true')
    p.add_argument('-f','--format',choices=['detailed','one-line'])
    p.add_argument('-c','--config',action='store_true',help='Edit saved configuration')
    args = p.parse_args()
    if args.config:
        edit_config()
    if args.list_models:
        for m in client.models.list().data: print(m.id)
        sys.exit(0)
    if args.format:
        COMMIT_FORMAT = args.format; save_fmt(COMMIT_FORMAT)
    stage(); d = diff()
    commit_msg = gen_msg(d)
    final = edit_inline(commit_msg)
    commit(final)
    push()


if __name__=='__main__':
    main()
