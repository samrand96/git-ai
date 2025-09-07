# CLI command to list models for the current provider
from core.config import settings
from providers.factory import get_provider

def main():
	provider_name = settings.get('PROVIDER', 'openai')
	provider = get_provider(provider_name)
	models = provider.list_models()
	print(f"Available models for {provider_name}:")
	for m in models:
		print(f"- {m}")

if __name__ == "__main__":
	main()
