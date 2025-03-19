#!/bin/bash

# Script to integrate vim runtime files into xvim
# This script copies all necessary runtime files from vim to xvim

# Exit on error
set -e

# Print what we're doing
set -x

# Define source and destination directories
VIM_RUNTIME="vim/runtime"
XVIM_RUNTIME="runtime"

# Create directories if they don't exist
mkdir -p "$XVIM_RUNTIME/syntax"
mkdir -p "$XVIM_RUNTIME/ftplugin"
mkdir -p "$XVIM_RUNTIME/indent"
mkdir -p "$XVIM_RUNTIME/colors"
mkdir -p "$XVIM_RUNTIME/compiler"
mkdir -p "$XVIM_RUNTIME/autoload"
mkdir -p "$XVIM_RUNTIME/plugin"
mkdir -p "$XVIM_RUNTIME/doc"
mkdir -p "$XVIM_RUNTIME/keymap"
mkdir -p "$XVIM_RUNTIME/spell"
mkdir -p "$XVIM_RUNTIME/tutor"
mkdir -p "$XVIM_RUNTIME/pack"
mkdir -p "$XVIM_RUNTIME/print"
mkdir -p "$XVIM_RUNTIME/lang"
mkdir -p "$XVIM_RUNTIME/tools"
mkdir -p "$XVIM_RUNTIME/macros"

# Copy core runtime files
cp "$VIM_RUNTIME/filetype.vim" "$XVIM_RUNTIME/"
cp "$VIM_RUNTIME/scripts.vim" "$XVIM_RUNTIME/"
cp "$VIM_RUNTIME/menu.vim" "$XVIM_RUNTIME/"
cp "$VIM_RUNTIME/defaults.vim" "$XVIM_RUNTIME/"
cp "$VIM_RUNTIME/synmenu.vim" "$XVIM_RUNTIME/"

# Copy syntax files
cp "$VIM_RUNTIME/syntax/"*.vim "$XVIM_RUNTIME/syntax/"

# Copy ftplugin files
cp "$VIM_RUNTIME/ftplugin/"*.vim "$XVIM_RUNTIME/ftplugin/"

# Copy indent files
cp "$VIM_RUNTIME/indent/"*.vim "$XVIM_RUNTIME/indent/"

# Copy color schemes
cp "$VIM_RUNTIME/colors/"*.vim "$XVIM_RUNTIME/colors/"

# Copy compiler files
cp "$VIM_RUNTIME/compiler/"*.vim "$XVIM_RUNTIME/compiler/"

# Copy autoload files
cp -r "$VIM_RUNTIME/autoload/"* "$XVIM_RUNTIME/autoload/"

# Copy plugin files
cp "$VIM_RUNTIME/plugin/"*.vim "$XVIM_RUNTIME/plugin/"

# Copy documentation
cp "$VIM_RUNTIME/doc/"*.txt "$XVIM_RUNTIME/doc/"

# Copy keymap files
cp "$VIM_RUNTIME/keymap/"*.vim "$XVIM_RUNTIME/keymap/"

# Copy spell files
cp "$VIM_RUNTIME/spell/"*.vim "$XVIM_RUNTIME/spell/"
cp "$VIM_RUNTIME/spell/"*.spl "$XVIM_RUNTIME/spell/" 2>/dev/null || true
cp "$VIM_RUNTIME/spell/"*.sug "$XVIM_RUNTIME/spell/" 2>/dev/null || true

# Copy tutor files
cp "$VIM_RUNTIME/tutor/"*.vim "$XVIM_RUNTIME/tutor/"
cp "$VIM_RUNTIME/tutor/"*.tutor "$XVIM_RUNTIME/tutor/" 2>/dev/null || true

# Copy tools
cp "$VIM_RUNTIME/tools/"*.vim "$XVIM_RUNTIME/tools/"

# Copy macros
cp -r "$VIM_RUNTIME/macros/"* "$XVIM_RUNTIME/macros/"

echo "Vim runtime files have been integrated into xvim successfully!"