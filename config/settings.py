# ...existing code...

import os
import platform
import configparser
from pathlib import Path
from typing import Any, Dict, Optional

class Settings:
	"""
	Central configuration manager for git-ai.
	Supports loading, saving, and updating settings for API, model, commit format, templates, hooks, language, etc.
	"""
	DEFAULTS = {
		'PROVIDER': 'openai',
		'COMMIT_FORMAT': 'detailed',
		'TEMPLATE': '',  # For future: custom commit templates
		'HOOKS_ENABLED': 'false',
		'LANGUAGE': 'en',
	}

	PROVIDER_DEFAULTS = {
		'openai': {
			'API_KEY': '',
			'MODEL': 'gpt-4o-mini',
		},
		'ollama': {
			'HOST': 'http://localhost:11434',
			'MODEL': 'llama3',
		},
		'anthropic': {
			'API_KEY': '',
			'MODEL': 'claude-3-opus-20240229',
		},
		'gemini': {
            'API_KEY': '',
			'MODEL': 'gemini-1.5-pro',
        },
		'lmstudio': {
            'HOST': 'http://localhost:1234',
			'MODEL': 'gemma-3-1b-it-qat',
        },
        'groq': {
            'API_KEY': '',
            'MODEL': 'openai/gpt-oss-120b',
        },
	}

	@classmethod
	def config_file_exists(cls) -> bool:
		"""Return True if the config file exists."""
		return cls.get_config_file().exists()
	@classmethod
	def config_is_valid(cls) -> bool:
		"""
		Returns True if the config file exists and required config values for the selected provider are present.
		"""
		if not cls.config_file_exists():
			return False
		# Load config to check required fields
		parser = configparser.ConfigParser()
		parser.read(cls.get_config_file())
		# Check global provider
		provider = parser['DEFAULT'].get('PROVIDER', 'openai').lower()
		# Required fields for each provider
		required = {
			'openai': ['API_KEY', 'MODEL'],
			'ollama': ['HOST', 'MODEL'],
			'anthropic': ['API_KEY', 'MODEL'],
			'gemini': ['API_KEY', 'MODEL'],
			'lmstudio': ['HOST', 'MODEL'],
            'groq': ['API_KEY', 'MODEL'],
		}
		section = provider.upper()
		if section not in parser:
			return False
		for key in required.get(provider, []):
			if not parser[section].get(key):
				return False
		return True

	@staticmethod
	def get_config_dir() -> Path:
		xdg = os.environ.get("XDG_CONFIG_HOME")
		if xdg:
			return Path(xdg)
		system = platform.system()
		home = Path.home()
		if system == 'Windows':
			return Path(os.getenv('APPDATA', home))
		return home / '.config'

	@classmethod
	def get_config_file(cls) -> Path:
		return cls.get_config_dir() / 'sam.git.ini'

	def __init__(self, config_file: Optional[Path] = None):
		self.config_file = config_file or self.get_config_file()
		self.config = configparser.ConfigParser()
		self._data: Dict[str, Any] = dict(self.DEFAULTS)
		self._provider_data: Dict[str, Dict[str, Any]] = {k: dict(v) for k, v in self.PROVIDER_DEFAULTS.items()}
		self.load()

	def load(self):
		if self.config_file.exists():
			self.config.read(self.config_file)
			# Load global/defaults
			for k in self.DEFAULTS:
				v = self.config['DEFAULT'].get(k, self.DEFAULTS[k])
				self._data[k] = v
			# Load provider sections
			for provider, defaults in self.PROVIDER_DEFAULTS.items():
				section = provider.upper()
				if section in self.config:
					for k, default in defaults.items():
						self._provider_data[provider][k] = self.config[section].get(k, default)
		else:
			self.save()  # Create with defaults

	def save(self):
		self.config['DEFAULT'] = {k: str(v) for k, v in self._data.items()}
		# Save provider sections
		for provider, pdata in self._provider_data.items():
			section = provider.upper()
			self.config[section] = {k: str(v) for k, v in pdata.items()}
		self.config_file.parent.mkdir(parents=True, exist_ok=True)
		with open(self.config_file, 'w') as f:
			self.config.write(f)

	def get(self, key: str, default: Any = None) -> Any:
		return self._data.get(key, default)

	def set(self, key: str, value: Any):
		self._data[key] = value
		self.save()

	def get_provider(self) -> str:
		return self._data.get('PROVIDER', 'openai')

	def get_provider_option(self, key: str, provider: Optional[str] = None, default: Any = None) -> Any:
		provider = provider or self.get_provider()
		return self._provider_data.get(provider, {}).get(key, default)

	def set_provider_option(self, key: str, value: Any, provider: Optional[str] = None):
		provider = provider or self.get_provider()
		if provider not in self._provider_data:
			self._provider_data[provider] = {}
		self._provider_data[provider][key] = value
		self.save()

	def as_dict(self) -> Dict[str, Any]:
		d = dict(self._data)
		d['providers'] = {k: dict(v) for k, v in self._provider_data.items()}
		return d

	# For future: support for templates, hooks, language, etc.
	# def get_template(self): ...
	# def set_template(self, template: str): ...
	# def enable_hooks(self, enabled: bool): ...
	# def set_language(self, lang: str): ...
