# Gemini LLM provider implementation using Google GenerativeAI Python library
import google.generativeai as genai
from .base import ProviderBase
from core.config import settings

class GeminiProvider(ProviderBase):
	def __init__(self, api_key: str = None, model: str = None, **kwargs):
		# Prefer explicit args, then provider config, then fallback
		self.api_key = api_key or settings.get_provider_option('API_KEY', 'gemini', '')
		self.model = model or settings.get_provider_option('MODEL', 'gemini', 'gemini-pro')
		genai.configure(api_key=self.api_key)
		self.client = genai.GenerativeModel(self.model)

	def generate(self, prompt: str, **kwargs):
		"""Generate a response using the Gemini Python SDK."""
		response = self.client.generate_content(prompt)
		try:
			return response.text
		except Exception:
			return str(response)

	def list_models(self):
		"""List available Gemini models (static list or via API if available)."""
		# The Python SDK does not provide a public model listing endpoint as of now
		return ["gemini-pro", "gemini-pro-vision"]
