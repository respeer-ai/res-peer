#!/bin/bash

cargo install --path service/copilot
cargo install --path service/illustrator

PROJECT_ROOT=/home/test/linera-project
AI_MODELS=$PROJECT_ROOT/ai-models
COEDIT_T5_XXL=$AI_MODELS/coedit-t5/t5_xxl

copilot-service --config-file $COEDIT_T5_XXL/config-xxl.json --weight-file $COEDIT_T5_XXL/model-xxl.gguf --tokenizer $COEDIT_T5_XXL/tokenizer.json > $PROJECT_ROOT/copilot-service.log 2>&1 &
illustrator-service --height "720" --width "720" --sd-version "xl"  > $PROJECT_ROOT/illustrator-service.log 2>&1
