# LM Studio LLM provider implementation
import requests
from .base import ProviderBase
from core.config import settings

class LMStudioProvider(ProviderBase):
    def __init__(self, host: str = None, model: str = None, **kwargs):
        # Prefer explicit args, then provider config, then fallback
        self.host = (host or settings.get_provider_option('HOST', 'lmstudio', 'http://localhost:1234')).rstrip('/')
        self.model = model or settings.get_provider_option('MODEL', 'lmstudio', 'default')

    def generate(self, prompt: str, **kwargs):
        """Generate a response using the LM Studio local API."""
        url = f"{self.host}/v1/completions"
        payload = {
            "model": self.model,
            "prompt": prompt,
            **kwargs
        }
        resp = requests.post(url, json=payload, timeout=60)
        resp.raise_for_status()
        result = resp.json()
        return result.get('choices', [{}])[0].get('text', '')

    def list_models(self):
        """List available LM Studio models (if API supports it, else static list)."""
        url = f"{self.host}/v1/models"
        try:
            resp = requests.get(url, timeout=10)
            resp.raise_for_status()
            data = resp.json()
            return [m['id'] for m in data.get('data', [])]
        except Exception:
            print("Could not fetch models from LM Studio API; returning configured model.")
            return [self.model]
