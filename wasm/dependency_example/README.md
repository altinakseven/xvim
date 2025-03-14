# Plugin Dependency Management Example

This directory contains example dependency files for the xvim plugin system. These files demonstrate how plugin dependencies are declared and managed.

## Dependency Files

The plugin system uses JSON files with the `.dep.json` extension to declare plugin dependencies. These files are placed alongside the plugin WASM files and are loaded when the plugin is loaded.

### Base Plugin

`base_plugin.dep.json` is the simplest example, representing a plugin with no dependencies:

```json
{
  "name": "base_plugin",
  "version": "1.0.0",
  "dependencies": []
}
```

### Dependent Plugin

`dependent_plugin.dep.json` represents a plugin that depends on the base plugin:

```json
{
  "name": "dependent_plugin",
  "version": "0.5.0",
  "dependencies": [
    {
      "name": "base_plugin",
      "version_req": "1.0.0",
      "optional": false
    }
  ]
}
```

This plugin requires `base_plugin` version `1.0.0` to be loaded before it can be loaded.

### Complex Plugin

`complex_plugin.dep.json` represents a plugin with multiple dependencies, including an optional one:

```json
{
  "name": "complex_plugin",
  "version": "1.2.0",
  "dependencies": [
    {
      "name": "base_plugin",
      "version_req": "1.0.0",
      "optional": false
    },
    {
      "name": "dependent_plugin",
      "version_req": "0.5.0",
      "optional": false
    },
    {
      "name": "optional_plugin",
      "version_req": "0.3.0",
      "optional": true
    }
  ]
}
```

This plugin requires both `base_plugin` and `dependent_plugin` to be loaded, but `optional_plugin` is optional and will be used if available.

## How Dependency Management Works

When a plugin is loaded, the plugin system:

1. Checks if a dependency file exists for the plugin
2. If it does, loads the dependency information
3. Checks if all required dependencies are installed and have compatible versions
4. Loads the dependencies in the correct order (dependencies before dependents)
5. Detects and prevents circular dependencies

The dependency manager uses topological sorting to determine the correct load order for plugins, ensuring that all dependencies are loaded before the plugins that depend on them.

## Benefits of Dependency Management

- **Modularity**: Plugins can be built on top of other plugins, promoting code reuse
- **Versioning**: Version requirements ensure compatibility between plugins
- **Optional Dependencies**: Plugins can use optional features if available
- **Load Order**: Automatic determination of the correct load order
- **Error Prevention**: Early detection of missing or incompatible dependencies

## Using Dependency Management in Your Plugins

To use dependency management in your plugins:

1. Create a `.dep.json` file alongside your plugin WASM file
2. Declare your plugin's name and version
3. List all dependencies with their required versions
4. Mark dependencies as optional if they're not required

The plugin system will handle the rest, ensuring that your plugin is loaded correctly with all its dependencies.