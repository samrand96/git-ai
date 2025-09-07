# Ollama LLM provider implementation


import requests
from .base import ProviderBase
from core.config import settings


class OllamaProvider(ProviderBase):
	def __init__(self, model: str = None, host: str = None, **kwargs):
		# Prefer explicit args, then provider config, then fallback
		self.model = model or settings.get_provider_option('MODEL', 'ollama', 'llama3')
		self.host = (host or settings.get_provider_option('HOST', 'ollama', 'http://localhost:11434')).rstrip('/')

	def generate(self, prompt: str, **kwargs):
		"""Generate a response using the Ollama local API."""
		url = f"{self.host}/api/generate"
		payload = {
			"model": self.model,
			"prompt": prompt,
			**kwargs
		}
		resp = requests.post(url, json=payload, timeout=60)
		resp.raise_for_status()
		# Ollama streams responses, but for simplicity, get the first chunk
		for line in resp.iter_lines():
			if line:
				import json
				data = json.loads(line)
				if 'response' in data:
					return data['response']
		return ""

	def list_models(self):
		"""List available Ollama models via the local API."""
		url = f"{self.host}/api/tags"
		resp = requests.get(url, timeout=10)
		resp.raise_for_status()
		data = resp.json()
		return [m['name'] for m in data.get('models', [])]
