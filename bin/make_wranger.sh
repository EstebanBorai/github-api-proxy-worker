#!/bin/bash


# read more about configuring your Worker via wrangler.toml at:
# https://developers.cloudflare.com/workers/cli-wrangler/configuration

# Environment Variables Required
# 
# - CF_ACCOUNT_ID: Account ID from Cloudflare
# - CF_ZONE_ID: Zone ID from Cloudflare

echo -e "name = 'website-api'\ntype = 'javascript'\nworkers_dev = false\ncompatibility_date = '2021-12-21'\n\n[vars]\nWORKERS_RS_VERSION = '0.0.7'\n\n[build]\ncommand = 'cargo install -q worker-build && worker-build --release'\n\n[build.upload]\ndir    = 'build/worker'\nformat = 'modules'\nmain   = './shim.mjs'\n\n[[build.upload.rules]]\nglobs = ['**/*.wasm']\ntype  = 'CompiledWasm'\n" > wrangler.toml
