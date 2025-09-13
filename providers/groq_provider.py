from .base import ProviderBase
from core.config import settings
import openai

class GroqProvider(ProviderBase):
	def __init__(self, api_key: str = None, model: str = None, **kwargs):

		self.api_key = api_key or settings.get_provider_option('API_KEY', 'groq', None)
		self.model = model or settings.get_provider_option('MODEL', 'groq', 'openai/gpt-oss-120b')
		self.client = openai.OpenAI(base_url="https://api.groq.com/openai/v1", api_key=self.api_key)

	def generate(self, prompt: str, **kwargs):
		messages = kwargs.get('messages')
		if not messages:
			messages = [{"role": "user", "content": prompt}]
		response = self.client.chat.completions.create(
			model=self.model,
			messages=messages,
			**{k: v for k, v in kwargs.items() if k != 'messages'}
		)
		return response.choices[0].message.content.strip()

	def list_models(self):
		return [m.id for m in self.client.models.list().data]
