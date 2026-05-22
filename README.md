# mAI-KB

## Overview

A simple CLI tool written in Rust that takes a local directory
and makes a knowledge base (KB) out of it, without messing with the local files.
The KB can be later consumed by AI agents, e.g. [mAI-consigliere](https://github.com/rafalkrol-xyz/mAI-consigliere).

### Usage

```bash
# Create a knowledge base
mkb up                                    # uses current directory, AWS by default
mkb up --dir /path/to/dir --provider gcp

# List knowledge bases
mkb ls
mkb ls --provider gcp

# Get info on a knowledge base
mkb get --id <kb-id>

# Sync a knowledge base
mkb sync
mkb sync --dir /path/to/dir --provider gcp

# Delete a knowledge base
mkb down --id <kb-id>

# Aliases: up → create, ls → list, down → rm / delete
```
