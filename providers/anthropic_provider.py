# Anthropic LLM provider implementation
from .base import ProviderBase
from core.config import settings
import anthropic

class AnthropicProvider(ProviderBase):
	def __init__(self, api_key: str = None, model: str = None, **kwargs):
		# Prefer explicit args, then provider config, then fallback
		self.api_key = api_key or settings.get_provider_option('API_KEY', 'anthropic', '')
		self.model = model or settings.get_provider_option('MODEL', 'anthropic', 'claude-3-opus-20240229')
		self.client = anthropic.Anthropic(api_key=self.api_key)

	def generate(self, prompt: str, **kwargs):
		messages = kwargs.get('messages')
		if not messages:
			messages = [{"role": "user", "content": prompt}]
		response = self.client.messages.create(
			model=self.model,
			messages=messages,
			**{k: v for k, v in kwargs.items() if k != 'messages'}
		)
		return response.content[0].text.strip() if hasattr(response.content[0], 'text') else str(response.content[0])

	def list_models(self):
		models = self.client.models.list(limit=20)
		return [m.id for m in models.data]
