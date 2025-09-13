# Provider factory for LLM providers
from .base import ProviderBase
from .openai_provider import OpenAIProvider
from .anthropic_provider import AnthropicProvider
from .ollama_provider import OllamaProvider
from .gemini_provider import GeminiProvider
from .lmstudio_provider import LMStudioProvider
from .groq_provider import GroqProvider

_PROVIDER_MAP = {
	'openai': OpenAIProvider,
	'anthropic': AnthropicProvider,
	'ollama': OllamaProvider,
    'gemini': GeminiProvider,
    'lmstudio': LMStudioProvider,
    'groq': GroqProvider
}

def get_provider(name: str, **kwargs) -> ProviderBase:
	"""Return an instance of the provider by name."""
	cls = _PROVIDER_MAP.get(name.lower())
	if not cls:
		raise ValueError(f"Unknown provider: {name}")
	return cls(**kwargs)
