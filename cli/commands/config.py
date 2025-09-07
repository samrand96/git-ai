# CLI config editor for global and provider-specific settings
from core.config import settings

def print_current():
	print("Current global settings:")
	for k, v in settings._data.items():
		print(f"  {k}: {v}")
	print("\nProvider settings:")
	for provider, pdata in settings._provider_data.items():
		print(f"[{provider}]")
		for k, v in pdata.items():
			print(f"  {k}: {v}")

def interactive_edit():
	print_current()
	print("\nEdit global settings (press Enter to keep current value):")
	for k in settings._data:
		val = input(f"{k} [{settings._data[k]}]: ").strip()
		if val:
			settings.set(k, val)
	print("\nEdit provider settings:")
	provider = settings.get_provider()
	provider_items = settings._provider_data.get(provider, {})
	from providers.factory import get_provider
	for k in provider_items:
		if k == 'MODEL':
			print(f"Available models for {provider}:")
			try:
				prov = get_provider(provider)
				models = prov.list_models()
				for m in models:
					print(f"  - {m}")
			except Exception as e:
				print(f"  [Error fetching models: {e}]")
		val = input(f"{provider}.{k} [{provider_items[k]}]: ").strip()
		if val:
			settings.set_provider_option(k, val, provider)
	print("Settings updated.")

def main():
	import argparse
	parser = argparse.ArgumentParser(description="Edit git-ai configuration.")
	parser.add_argument('--interactive', action='store_true', help='Edit config interactively')
	parser.add_argument('--set', nargs=2, metavar=('KEY', 'VALUE'), help='Set a global config value')
	parser.add_argument('--set-provider', nargs=3, metavar=('PROVIDER', 'KEY', 'VALUE'), help='Set a provider config value')
	args = parser.parse_args()

	if args.interactive or (not args.set and not args.set_provider):
		interactive_edit()
	if args.set:
		k, v = args.set
		settings.set(k, v)
		print(f"Set {k} = {v}")
	if args.set_provider:
		provider, k, v = args.set_provider
		settings.set_provider_option(k, v, provider)
		print(f"Set [{provider}].{k} = {v}")

if __name__ == "__main__":
	main()
